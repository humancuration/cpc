#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

@interface CPCThumbnailGenerator : NSObject

- (BOOL)generateThumbnailForModelAtPath:(NSString *)modelPath
                             outputPath:(NSString *)outputPath
                                  size:(NSUInteger)size
                                 error:(NSError **)error;

@end

NS_ASSUME_NONNULL_END