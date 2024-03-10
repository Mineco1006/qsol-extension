export type AuthenticationRequest = {
  service_name: string;
  timestamp: number;
  id: string;
  signature: string;
};

declare global {
  interface Window {
      qweb3: object | undefined;
  }
}

export class QsolExtension {
  timeout: number = 60000;

  public constructor() {}
  
  isActive(): boolean {
    return window.qweb3 !== undefined;
  }

  setRequestTimeout(ms: number) {
    this.timeout = ms;
  }

  private call<T>(messageType: string, params: any): Promise<T> {
    return new Promise((resolve, reject) => {
      window.postMessage(
        { type: messageType, timeout: this.timeout, ...params },
        "*",
      );

      window.addEventListener("message", (event) => {
        switch (event.data.type) {
          case "rejected":
            reject("Request has been rejected!");
            break;
          case "accepted":
            resolve(event.data.value);
            break;
        }
      });

      setTimeout(() => {
        reject("Request timed out!");
      }, this.timeout);
    });
  }

  requestAccounts(): Promise<string[]> {
    return this.call("requestAccounts", {});
  }

  requestAuthentication(service_name: string): Promise<AuthenticationRequest> {
    return this.call("requestAuthentication", {
      service_name: service_name,
    });
  }

  sendTransaction(to: string, amount: number, input_type = 0, input_size = 0) {
    return this.call("sendTransaction", {
      to: to,
      amount: amount,
      input_type: input_type,
      input_size: input_size,
    });
  }
}
