#import "CPCThumbnailGenerator.h"
#include "cpc_core.h"

@implementation CPCThumbnailGenerator

- (BOOL)generateThumbnailForModelAtPath:(NSString *)modelPath
                             outputPath:(NSString *)outputPath
                                  size:(NSUInteger)size
                                 error:(NSError **)error {
    const char *model_path = [modelPath UTF8String];
    const char *output_path = [outputPath UTF8String];
    
    const char *error_msg = generate_model_thumbnail(model_path, output_path, (unsigned int)size);
    
    if (error_msg != NULL) {
        NSString *errorStr = [NSString stringWithUTF8String:error_msg];
        *error = [NSError errorWithDomain:@"CPCThumbnailGenerator"
                                     code:1
                                 userInfo:@{NSLocalizedDescriptionKey: errorStr}];
        return NO;
    }
    
    return YES;
}

@end