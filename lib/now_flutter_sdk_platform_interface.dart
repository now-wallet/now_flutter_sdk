import 'package:plugin_platform_interface/plugin_platform_interface.dart';

import 'now_flutter_sdk_method_channel.dart';

abstract class NowFlutterSdkPlatform extends PlatformInterface {
  /// Constructs a NowFlutterSdkPlatform.
  NowFlutterSdkPlatform() : super(token: _token);

  static final Object _token = Object();

  static NowFlutterSdkPlatform _instance = MethodChannelNowFlutterSdk();

  /// The default instance of [NowFlutterSdkPlatform] to use.
  ///
  /// Defaults to [MethodChannelNowFlutterSdk].
  static NowFlutterSdkPlatform get instance => _instance;

  /// Platform-specific implementations should set this with their own
  /// platform-specific class that extends [NowFlutterSdkPlatform] when
  /// they register themselves.
  static set instance(NowFlutterSdkPlatform instance) {
    PlatformInterface.verifyToken(instance, _token);
    _instance = instance;
  }

  Future<String?> getPlatformVersion() {
    throw UnimplementedError('platformVersion() has not been implemented.');
  }
}
