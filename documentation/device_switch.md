# Device Switch

Programmatically setting a an audio session's input/output device is not a documented feature.
However, the audio mixer in the settings menu can obviously do just that.
I want to do that, too.

## Reverse Engineering

Using `procexp`, I found the settings executable - `SystemSettings.exe`.

### SystemSettings.exe

`SystemSettings.exe` is a pretty small executable that loads `SystemSettings.dll` which contains most of the logic.

### SystemSettings.dll

I started looking for strings with the term `audio`, `volume`, etc. in the library and found `ms-settings:apps-volume`.

It turns out that the settings has a URI "protocol" - for example, accessing the path `ms-settings:apps-volume` opens the volume mixer directly. Each link has a name, and this one's name is `SystemSettings_Audio_MixerLink`.

I didn't see anything useful immediately, and reversing this library would've been a nightmare, so I transitioned to `procmon`.

When clicking on the `Sound` option in the setting's main menu, a registry value is read from the path:

- Path: `HKLM\SOFTWARE\Microsoft\SystemSettings\SettingId\SystemSettings_Audio_OutputCollectionHeader`
- Name: `DllPath`
- Type: `REG_SZ`
- Value: `C:\Windows\System32\AudioHandlers.dll`

There are many `SystemSettings_Audio_*` keys, which all point to `AudioHandlers.dll`.
This appears to be a common pattern, as all the `SystemSettings_AssignedAccess_*` keys point to the same library too, `SettingsHandlers_AssignedAccess.dll`.

### AudioHandlers.dll

`AudioHandlers.dll`, being the actual audio mixer, has A LOT of logic related to audio devices and sessions - session enumeration, setting session volume, setting a session's output device, etc.
In short, `SystemSettings::DataModel::SessionInfoItem` is the object that represents a single stream, and it's methods are used to view and modify it.

The `SetPersistedDefaultAudioEndpoint` is the function responsible for setting a session's input or output device.
It just calls the `AudioPolicyConfigFactory::SetPersistedDefaultAudioEndpoint` method from `AudioSes.dll`.
The `AudioPolicyConfigFactory` is received over COM.

Now I have two options.

1. Take inspiration from the `EarTrumpet` project, which I just found, that receives an instance of the `AudioPolicyConfigFactory` class over COM.
2. Reverse `AudioSes.dll` and figure out how it really works, and just do that directly.

You already know it's going to be option 2.

### AudioSes.dll

`AudioPolicyConfigFactory::SetPersistedDefaultAudioEndpoint` method gets the audio policy config singleton and calls `AudioPolicyConfig::SetPersistedDefaultAudioEndpoint`, which interacts with the audio service, `Audiosrv`, via RPC (procedure `0x24`).

### audiosrv.dll

`audiosrv.dll` is the library for `Audiosrv`. As seen before, it provides an RPC interface.
We are interested in procedure `0x24`, which is `s_apmSetPersistedDefaultAudioEndpoint`.

The library contains a global of type `IAudioPolicyManager`, which the method uses to receive an object of type `IApplicationSpecificEndpointInfo` (this object is fetched by process ID).
This object, too, has a `SetPersistedDefaultAudioEndpoint` method.

`IApplicationSpecificEndpointInfo` is an interface type, and the actual type seems to be `ApplicationSpecificEndpointInfo` from `AudioSrvPolicyManager.dll`.

### AudioSrvPolicyManager.dll

`ApplicationSpecificEndpointInfo::SetPersistedDefaultAudioEndpoint` modifies values under `HKEY_CURRENT_USER\Software\Microsoft\Multimedia\Audio\DefaultEndpoint`.

Each process has it's own subkey: `HKEY_CURRENT_USER\Software\Microsoft\Multimedia\Audio\DefaultEndpoint\{ProcessNameHash}_{CollisionCount}`. The hash is very simple:

```python
def hash(string: str) -> int:
    result = 0
    for character in string:
        result = (ord(character) + ((0x21 * result) & 0xFFFFFFFF)) & 0xFFFFFFFF
    return result
```

For each "modification" over the global default devices, several values are created:

- Name: `{Role}_{DataFlow}`
- Value: `\\?\SWD#MMDEVAPI#{0.0.0.00000000}.{GUID1}#{GUID2}`

- Name: `{Role}_{DataFlow}_p`
- Value: `{GUID}`

`Role` and `DataFlow` are always represented as `%03d`, and GUIDs are represented as `{XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX}`.

The first value is a symbolic link to the audio device. The second one is a bullshit GUID.

Later, `ApplicationSpecificEndpointInfo::UpdateState` is called and updates values in it's resource policy arrays.

## Conclusions

I think that, generally, this is it - the audio service owns the private state (the resource policy arrays in `ApplicationSpecificEndpointInfo`), which are backed by the registry at `HKEY_CURRENT_USER\Software\Microsoft\Multimedia\Audio\DefaultEndpoint`.

Here is the flow from the settings to the audio device modification.

| Executable / DLL | Description | Flow |
| - | - | - |
| `SystemSettings.exe` | The Windows settings executable. | Loads `SystemSettings.dll`, which contains most of the logic. |
| `SystemSettings.dll` | The Windows settings library. | Loads `AudioHandlers.dll`, the volume mixer library, when the user enters the volume mixer for the first time. |
| `AudioHandlers.dll` | The Windows settings audio mixer library. | `SessionInfoItem::SetPersistedDefaultAudioEndpoint` calls `AudioPolicyConfigFactory::SetPersistedDefaultAudioEndpoint`. An instance of `AudioPolicyConfigFactory` is received from `AudioSes.dll` over COM. |
| `AudioSes.dll` | Audio session management COM library. | `AudioPolicyConfigFactory::SetPersistedDefaultAudioEndpoint` calls `AudioPolicyConfig::SetPersistedDefaultAudioEndpoint` on a `AudioPolicyConfig` singleton, which calls `Audiosrv`'s procedure `0x24` - `s_apmSetPersistedDefaultAudioEndpoint` - over RPC. |
| `Audiosrv.dll` | The Windows audio service. | `s_apmSetPersistedDefaultAudioEndpoint` uses a global `IAudioPolicyManager` object to get a `IApplicationSpecificEndpointInfo` for the target process, and calls `IApplicationSpecificEndpointInfo::SetPersistedDefaultAudioEndpoint`. `IApplicationSpecificEndpointInfo` is an interface, the instance is of type `ApplicationSpecificEndpointInfo`, from `AudioSrvPolicyManager.dll` |
| `AudioSrvPolicyManager.dll` | Policy management library for the Windows audio service | Manages lists of policies, which describes application-to-device association, and are backed by the registry. |

I COULD modify the registry and edit `AudioSrvPolicyManager.dll`'s lists in memory, but that would be insane.
The best way for me to do what I'm trying to do is either:

1. Use the `AudioSes.dll` COM interface.
2. Use the `Audiosrv` RPC interface.
