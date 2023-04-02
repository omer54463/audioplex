# Notification Callbacks

The Windows audio ecosystem provides many callback interfaces for programmers.
I need to pick the ones I need.

## Required Events

These are the events I would like to receive a callback for.

- Device addition.
- Device removal.
- Device state change.
- Device name change.
- Device icon change.
- Session addition.
- Session removal.
- Session state change.
- Session name change.
- Session icon change.
- Session volume change.

## Callback Interfaces Overview

| Interface | Method | Callback Interface | Notifications |
| - | - | - | - |
| `IAudioEffectsManager` | `RegisterAudioEffectsChangedNotificationCallback` | `IAudioEffectsChangedNotificationClient ` | Changes to audio effects list. |
| `IAudioSessionControl` | `RegisterAudioSessionNotification` | `IAudioSessionEvents` | Session removal/state change/name change/icon change/volume change. |
| `IAudioSessionControl2` | `RegisterAudioSessionNotification` | `IAudioSessionEvents` | Session removal/state change/name change/icon change/volume change. |
| `IAudioSessionManager2` | `RegisterSessionNotification` | `IAudioSessionNotification` | Session addition. |
| `IAudioSessionManager2` | `RegisterDuckNotification` | `IAudioVolumeDuckNotification` | Stream attenuation, or ducking, events. |
| `IAudioStateMonitor` | `RegisterCallback` | NDA protected. | NDA protected. |
| `IAudioSystemEffectsPropertyStore` | `RegisterPropertyChangeNotification` | `IAudioSystemEffectsPropertyChangeNotificationClient` | System effect property changes. |
| `IMMDeviceEnumerator` | `RegisterEndpointNotificationCallback` | `IMMNotificationClient` | Device addition/removal/state change/name change/icon change. |
| `IPart` | `RegisterControlChangeCallback` | `IControlChangeNotify` | Connector or subunit changes. |

## Relevant Callback Interfaces

- `IAudioSessionControl2::RegisterAudioSessionNotification`
- `IAudioSessionManager2::RegisterSessionNotification`
- `IMMDeviceEnumerator::RegisterEndpointNotificationCallback`
