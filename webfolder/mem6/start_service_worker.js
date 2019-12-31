if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('service_worker.js').then(function (registration) {
        console.log('Registration succeeded.');
    }).catch(function (error) {
        console.log('Registration failed with ' + error);
    });
};
