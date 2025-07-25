// Camera initialization for barcode scanning
window.camera = {
    startCamera: function(videoId) {
        return new Promise((resolve, reject) => {
            const video = document.getElementById(videoId);
            if (!video) {
                reject('Video element not found');
                return;
            }
            
            if (navigator.mediaDevices && navigator.mediaDevices.getUserMedia) {
                navigator.mediaDevices.getUserMedia({ video: { facingMode: "environment" } })
                    .then(function(stream) {
                        video.srcObject = stream;
                        video.play()
                            .then(() => resolve())
                            .catch(e => reject(`Video play error: ${e}`));
                    })
                    .catch(e => reject(`Camera access error: ${e}`));
            } else {
                reject('getUserMedia not supported');
            }
        });
    },
    
    requestCameraPermission: function() {
        return new Promise((resolve) => {
            if (!navigator.permissions) {
                resolve('prompt');
                return;
            }
            
            navigator.permissions.query({name: 'camera'})
                .then(permissionStatus => {
                    resolve(permissionStatus.state);
                })
                .catch(() => resolve('prompt'));
        });
    },
    
    checkCameraAvailability: function() {
        return new Promise((resolve) => {
            if (!navigator.mediaDevices || !navigator.mediaDevices.enumerateDevices) {
                resolve(false);
                return;
            }
            
            navigator.mediaDevices.enumerateDevices()
                .then(devices => {
                    const hasCamera = devices.some(device =>
                        device.kind === 'videoinput'
                    );
                    resolve(hasCamera);
                })
                .catch(() => resolve(false));
        });
    }
};