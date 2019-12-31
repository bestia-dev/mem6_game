///javascript function to find out if is mobile device
///maybe is not 100%, but good enought
export function javascriptismobiledevice() {
    return (typeof window.orientation !== "undefined") || (navigator.userAgent.indexOf('IEMobile') !== -1);
}
