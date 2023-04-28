import 'package:flutter_test/flutter_test.dart';
import 'package:now_flutter_sdk/now_flutter_sdk.dart';
import 'package:now_flutter_sdk/now_flutter_sdk_platform_interface.dart';
import 'package:now_flutter_sdk/now_flutter_sdk_method_channel.dart';
import 'package:plugin_platform_interface/plugin_platform_interface.dart';

class MockNowFlutterSdkPlatform
    with MockPlatformInterfaceMixin
    implements NowFlutterSdkPlatform {

  @override
  Future<String?> getPlatformVersion() => Future.value('42');
}

void main() {
  final NowFlutterSdkPlatform initialPlatform = NowFlutterSdkPlatform.instance;

  test('$MethodChannelNowFlutterSdk is the default instance', () {
    expect(initialPlatform, isInstanceOf<MethodChannelNowFlutterSdk>());
  });

  test('getPlatformVersion', () async {
    NowFlutterSdk nowFlutterSdkPlugin = NowFlutterSdk();
    MockNowFlutterSdkPlatform fakePlatform = MockNowFlutterSdkPlatform();
    NowFlutterSdkPlatform.instance = fakePlatform;

    expect(await nowFlutterSdkPlugin.getPlatformVersion(), '42');
  });
}
