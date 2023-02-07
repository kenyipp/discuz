# Actix Helmet

The `actix-helmet` is a Rust library that offers security enhancements for your Actix-web application by setting important HTTP headers. Inspired by the popular [helmet](https://www.npmjs.com/package/helmet) module in Node.js, this crate helps you safeguard your app from various security threats.

## Quick start
To install the actix-helmet crate using cargo, run the following command in your terminal or command prompt:
```sh
cargo add actix-helmet
```
Then, in your actix-web:
```rust
use actix_helmet::Helmet;
use actix_web::{web, App, HttpServer};

#[actix_web::main]
async fn main() {
	let port = 3000;
	HttpServer::new(move || {
		let helmet = Helmet::default();
		App::new().wrap(helmet).route(
			"/index.html",
			web::get().to(|| async { "Hello, middleware!" }),
		)
	})
	.bind(("127.0.0.1", port))
	.unwrap()
	.run()
	.await
	.unwrap();
}
```

The actix-helmet crate in Rust is similar to the helmet plugin in Node.js. By default, actix-helmet sets the following security-related HTTP headers to help protect your application:

```
Referrer-Policy: no-referrer
Strict-Transport-Security: max-age=15552000; includeSubDomains
X-Content-Type-Options: nosniff
X-DNS-Prefetch-Control: off
X-Download-Options: noopen
X-Frame-Options: SAMEORIGIN
X-Permitted-Cross-Domain-Policies: none
X-XSS-Protection: 0
```

I suggest using the [actix-cors](https://crates.io/crates/actix-cors) crate to handle CORS header management.

## Customization
Here is an explanation of various HTTP headers and how to set custom values using actix-helmet. I learned about most of the headers through [ChatGPT](https://chat.openai.com/chat).

### Strict-Transport-Security

Disable the Strict-Transport-Security header
```rust
let helmet = Helmet::default().disable_strict_transport_security();
```

Enable the Strict-Transport-Security header with a max age of 1 year, include it in subdomains, and disable preloading
```rust
let helmet = Helmet::default().enable_strict_transport_security(1000, true, false);
```

Strict-Transport-Security (often abbreviated as HSTS) is an HTTP header that helps to secure web applications by declaring that the browser should only communicate with a website using a secure HTTPS connection and never using an insecure HTTP connection. This helps to prevent **man-in-the-middle attacks** and protects sensitive information such as passwords and credit card numbers from being intercepted and read by malicious actors.

When a web browser receives an HSTS header from a website, it will remember that the site requires HTTPS for a specified amount of time, typically measured in seconds. The next time the user visits the site, the browser will automatically use HTTPS, even if the user types "http://" into the address bar or clicks on a link that uses HTTP.

The value of HSTS specifies the duration that the browser should remember that the site requires HTTPS and not use HTTP. A typical "Strict-Transport-Security" header might look like this:

`Strict-Transport-Security: max-age=31536000; includeSubDomains; preload`

 - `max-age`: Sets the duration in seconds that the browser should remember that the site requires HTTPS. It is typically set to a high value such as 31536000 (1 year), to ensure that the browser continues to use HTTPS for a significant amount of time.
 - `includeSubDomains`: Indicates that all subdomains of the site should also use HTTPS.
 - `preload`: Signals that the site is interested in being included in the HSTS preload list maintained by browser vendors, so that the site can be automatically considered to be "HTTPS only" by supporting browsers.

#### What is man in the middle (MITM) attack?
A man-in-the-middle (MITM) attack is a type of cyber attack where the attacker intercepts and alters communication between two parties without their knowledge. The attacker essentially sits between the two parties, acting as a "man in the middle," and can eavesdrop, modify, or block the communication.

MITM attacks can occur in various forms, such as eavesdropping on a public Wi-Fi network, compromising a router, or using a malicious proxy server. In these cases, the attacker is able to intercept and read sensitive information, such as login credentials and financial information, that is being transmitted between the two parties.

To protect against MITM attacks, it's important to use encryption, such as SSL/TLS, when transmitting sensitive information. This makes it much more difficult for attackers to intercept and read the information, as the encrypted data will appear as garbled text to anyone who tries to eavesdrop. Additionally, it's important to be wary of public Wi-Fi networks and to only connect to trusted networks whenever possible.

### Referrer-Policy
The `Referrer-Policy` header is a HTTP header that allows a website to control the values of the `Referer` header that is sent in requests from the site to other sites.  The `Referer` header provides information about the origin of the request, including the URL of the page that made the request.  
By using the `Referrer-Policy` header, a website can decide what information to include in the `Referer` header, or whether to include the header at all.   
For example, a site might choose to only include the origin of the request, or to not include the Referer header in requests to other sites.
The Referrer-Policy header is used to control the privacy of users, as the Referer header can contain sensitive information, such as search terms or the URL of a page containing a form. By limiting the information that is included in the Referer header, a site can prevent the privacy of its users from being compromised.  
Also, it's important to keep in mind that the behavior of the Referer header is determined by the browser, and the Referrer-Policy header is **just a way for the server to suggest a policy to the browser**. Some browsers may choose to ignore the server's policy and send the Referer header regardless. Therefore, it's important to test the behavior of the Referer header in your specific use case to determine if it is being sent as you expect.

The values that the Referrer-Policy header can take are as follows:

 - `no-referrer`: If you set the value "no-referrer" to the Referrer-Policy header in the HTTP response of your server, it will instruct the browser not to send any information about the origin of the request in the Referer header for subsequent HTTP requests made to **your server** from **the same page**. 
 - `no-referrer-when-downgrade`: Instructs the browser to send a full referrer in all cases, except when navigating from a secure (HTTPS) to a less secure (HTTP) URL.
 - `origin`: If the Referrer-Policy header is set to "origin", the browser will only send the origin (scheme, host, and port) of the page in the Referer header, and will never send the full path of the page. The browser will send the referrer regardless of whether the request is same-origin or cross-origin. 
 - `origin-when-cross-origin`: Instructs the browser to send only the origin (scheme, host, and port) of the page in the Referer header, not the full URL, when the request is cross-origin. If the request is same-origin, the full URL of the page will be sent in the Referer header. 
 - `same-origin`: Instructs the browser include the full path of the page in the Referer header if the request is made to a resource with the same origin. If the request is cross-origin, the Referer header will not be sent at all.
 - `strict-origin`: Instructs the browser to only send the origin information (scheme, host, and port) of the page in the Referer header, but only if the request is made to a secure origin (HTTPS). It is worth noting that only the origin information will be sent, and the full path of the page will never be included in the Referer header.
 - `If the Referrer-Policy header is set to`: Instructs the browser to send only the origin (scheme, host, and port) of the page in the Referer header if the request is cross-origin and made to a secure origin (HTTPS). If the request is made to a non-secure origin (HTTP) or if it is same-origin, the Referer header will not be sent and the full path of the page will not be included. Note that for same-origin requests, the origin information will not be included in the Referer header, even if the Referrer-Policy header is set to "strict-origin-when-cross-origin".
 - `unsafe-url`: Instruct the browser send the complete URL (scheme, host, port, and path) in the Referer header, regardless of whether the request is same-origin or cross-origin. This value is considered as "unsafe" because it exposes more information about the page than necessary and can potentially reveal sensitive information to malicious websites.

#### Summary

| Referrer-Policy                 | Description                                                                                                                                                                      | Same-Origin Requests | Cross-Origin Requests |
| ------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------- | --------------------- |
| no-referrer                     | No referrer information will be sent.                                                                                                                                            | No                   | No                    |
| no-referrer-when-downgrade      | No referrer information will be sent if the request is made from HTTPS to HTTP.                                                                                                  | Yes                  | No                    |
| same-origin                     | Only the origin (scheme, host, and port) of the page will be sent as the referrer.                                                                                               | Yes                  | No                    |
| origin                          | Only the origin (scheme, host, and port) of the page will be sent as the referrer.                                                                                               | Yes                  | Yes                   |
| strict-origin                   | Only the origin (scheme, host, and port) of the page will be sent as the referrer, but only if the request is made to a secure origin (HTTPS).                                   | No                   | No                    |
| origin-when-cross-origin        | Only the origin (scheme, host, and port) of the page will be sent as the referrer if the request is cross-origin. The full URL of the page will be sent in same-origin requests. | Yes                  | Yes                   |
| strict-origin-when-cross-origin | Only the origin (scheme, host, and port) of the page will be sent as the referrer if the request is cross-origin and made to a secure origin (HTTPS).                            | No                   | No                    |
| unsafe-url                      | The full URL of the page will be sent as the referrer, regardless of the request being same-origin or cross-origin.                                                              | Yes                  | Yes                   |

### X-XSS-Protection
The X-XSS-Protection header is a mechanism used to enhance the security of web applications by enabling the built-in cross-site scripting (XSS) protection in specific web browsers. By setting this header, you can help prevent malicious scripts from being executed, providing a barrier against XSS attacks.

The header can have one of the following values:

 - `0`: disables the XSS filter.
 - `1`: enables the XSS filter and will attempt to block the attack.
 - `1; mode=block`: enables the XSS filter and will block the response if a XSS attack is detected.
 - `1; report=<reporting-URI>`: enables the XSS filter and sends a report to the specified URI if a XSS attack is detected.

#### What is XSS attacks?
Cross-Site Scripting (XSS) is a type of security vulnerability that affects web applications. XSS attacks occur when an attacker **injects malicious scripts into a web page** that is then viewed by other users. When a user visits the affected page, the malicious scripts are executed in their browser, allowing the attacker to steal sensitive information, such as login credentials, or perform other malicious actions on behalf of the user.

There are two main categories of XSS attacks: Stored XSS and Reflected XSS:
 - `Stored XSS attacks` occur when the malicious script is stored on the server and served to every user who visits the affected page. 
 - `Reflected XSS attacks` occur when the malicious script is included in the URL and is reflected back to the user in the response.

XSS attacks can have serious consequences for both users and the web application. To prevent XSS attacks, web developers should carefully validate and sanitize user input and ensure that any dynamic content is properly escaped before it is included in the page. 

### X-Frame-Options
It is a HTTP header that provides clickjacking protection for a web page. It is used to indicate whether or not a browser should be allowed to render a page within an iframe or frame.

The header can be set to one of three values: 

 - `DENY`: Prevents the page from being displayed in a frame regardless of the source
 - `SAMEORIGIN`: Allows the page to be displayed in a frame only on the same origin
 - `ALLOW-FROM uri`: Allows the page to be displayed in a frame on the specified origin

### X-Download-Options
It is an HTTP header that provides protection against certain types of malicious attacks that use "File Download" dialog boxes to trick users into downloading malicious files. 
This header tells Internet Explorer 8 and later versions to prevent the "File Download" dialog box from appearing when a user downloads a file from the website. Instead, the file will be automatically downloaded to the user's default downloads directory.

### X-Permitted-Cross-Domain-Policies
It is an HTTP header used to control the behavior of Adobe Flash Player when it makes a request to a cross-domain resource. 

The header can be set to one of three values: 

 - `all`: Allow all cross-domain requests.
 - `none`: Do not allow any cross-domain requests
 - `master-only`: Only allow cross-domain requests for a SWF file designated as a master SWF file
 - `by-content-type`: Allow cross-domain requests based on the MIME type of the requested resource
 - `by-ftp-filename`: Allow cross-domain requests based on the file extension of the requested resource

#### What is cross domain request?
A cross-origin request, is an HTTP request made from a web page or script to a different domain than the one that served the web page. Browsers enforce a same-origin policy, which means that a web page or script can only access resources on the same domain as the web page. Cross-domain requests are necessary in some cases, such as when you want to access an API hosted on a different domain, or when you want to embed content from another domain in your web page. However, they can also introduce security risks, as they can allow an attacker to access sensitive data on another domain. To mitigate these risks, modern browsers implement mechanisms like the Same-Origin Policy and the Cross-Origin Resource Sharing (CORS) mechanism, which allow web administrators to control the behavior of cross-domain requests.

#### What is the allow mime type values of `by-content-type`?
The exact set of MIME types that are allowed in this case is determined by the web administrator and may vary depending on the specific use case and security requirements. It's worth noting that the X-Permitted-Cross-Domain-Policies header is not widely used, and CORS (Cross-Origin Resource Sharing) is now the preferred mechanism for controlling cross-domain requests in modern browsers. 

To effectively handle the Cross-Origin Resource Sharing values for your server, consider utilizing the [Actix Cors](https://crates.io/crates/actix-cors) library.

### X-DNS-Prefetch-Control
It is a http header used to control the DNS prefetching behavior of a web browser. DNS prefetching is a technique used by modern browsers to resolve domain names to IP addresses in the background, before the user clicks on a link. This can improve the performance of your website by reducing the time it takes for a user to access a linked resource.

The X-DNS-Prefetch-Control header can have two values:
 - `on`: this enables DNS prefetching.
 - `off`: this disables DNS prefetching.

By default, most modern browsers enable DNS prefetching. However, web administrators can use this header to disable DNS prefetching if they determine that it's causing performance problems or security issues.

### X-Content-Type-Options
It is a http header that tell the browser whether or not to process the response as a content type that the server did not specify in the header. By setting the value of this header to "nosniff", it instructs the browser not to sniff the MIME type and only use the one specified in the header. This helps to prevent certain types of cross-site scripting (XSS) attacks.
