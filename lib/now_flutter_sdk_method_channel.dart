import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

import 'now_flutter_sdk_platform_interface.dart';

/// An implementation of [NowFlutterSdkPlatform] that uses method channels.
class MethodChannelNowFlutterSdk extends NowFlutterSdkPlatform {
  /// The method channel used to interact with the native platform.
  @visibleForTesting
  final methodChannel = const MethodChannel('now_flutter_sdk');

  @override
  Future<String?> getPlatformVersion() async {
    final version = await methodChannel.invokeMethod<String>('getPlatformVersion');
    return version;
  }
}
