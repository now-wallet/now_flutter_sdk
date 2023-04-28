#import "NowPlugin.h"
#if __has_include(<now_flutter_sdk/now_flutter_sdk-Swift.h>)
#import <now_flutter_sdk/now_flutter_sdk-Swift.h>
#else
#import "now_flutter_sdk-Swift.h"
#endif

@implementation NowFlutterSdkPlugin
+ (void)registerWithRegistrar:(NSObject<FlutterPluginRegistrar>*)registrar {
  [SwiftNowFlutterSdkPlugin registerWithRegistrar:registrar];
}
@end