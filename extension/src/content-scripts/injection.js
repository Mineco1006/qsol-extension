console.log("Injecting qubic-web3 provider");

var script = document.createElement("script");



script.innerHTML = '(' + function () {
    window.qweb3 = {
        timeout: 60000,

        qu: {
            setRequestTimeout: (ms) => {
                window.qweb3.timeout = ms;
            },
            requestAccounts: () => {
                return new Promise((resolve, reject) => {
                    window.postMessage({ type: "requestAccounts", timeout: window.qweb3.timeout }, "*");

                    window.addEventListener("message", event => {
                        switch (event.data.type) {
                            case "rejected":
                                reject("Account request has been rejected!");
                                break;
                            case "accepted":
                                resolve(event.data.value);
                                break;
                        }
                    });

                    setTimeout(() => {
                        reject("Request timed out!")
                    }, window.qweb3.timeout);
                });
            },
            requestAuthentication: (service_name) => {
                return new Promise((resolve, reject) => {
                    window.postMessage({ type: "requestAuthentication", timeout: window.qweb3.timeout, service_name: service_name }, "*");

                    window.addEventListener("message", event => {
                        switch (event.data.type) {
                            case "rejected":
                                reject("Account request has been rejected!");
                                break;
                            case "accepted":
                                resolve(event.data.value);
                                break;
                        }
                    });

                    setTimeout(() => {
                        reject("Request timed out!")
                    }, window.qweb3.timeout);
                });
            },
            sendTransaction: (to, amount, input_type = 0, input_size = 0) => {
                return new Promise((resolve, reject) => {
                    window.postMessage({ type: "sendTransaction", timeout: window.qweb3.timeout, to: to, amount: amount, input_size: input_size, input_type: input_type }, "*");

                    window.addEventListener("message", event => {
                        switch (event.data.type) {
                            case "rejected":
                                reject("Transaction signature request has been rejected!");
                                break;
                            case "accepted":
                                resolve(event.data.value);
                                break;
                        }
                    });

                    setTimeout(() => {
                        reject("Request timed out!")
                    }, window.qweb3.timeout);
                });
            }
        }
        
        
    }
} + ')();';

document.head.appendChild(script);