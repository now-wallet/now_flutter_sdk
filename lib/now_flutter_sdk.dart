
import 'now_flutter_sdk_platform_interface.dart';

class NowFlutterSdk {
  Future<String?> getPlatformVersion() {
    return NowFlutterSdkPlatform.instance.getPlatformVersion();
  }
}
