# Actix Helmet

The `actix-helmet` is a Rust library that offers security enhancements for your Actix-web application by setting important HTTP headers. Inspired by the popular [helmet](https://www.npmjs.com/package/helmet) module in Node.js, this crate helps you safeguard your app from various security threats

### X-XSS-Protection
The X-XSS-Protection header is a mechanism used to enhance the security of web applications by enabling the built-in cross-site scripting (XSS) protection in specific web browsers. By setting this header, you can help prevent malicious scripts from being executed, providing a barrier against XSS attacks.

The header can have one of the following values:

 - `0`: disables the XSS filter.
 - `1`: enables the XSS filter and will attempt to block the attack.
 - `1; mode=block`: enables the XSS filter and will block the response if a XSS attack is detected.
 - `1; report=<reporting-URI>`: enables the XSS filter and sends a report to the specified URI if a XSS attack is detected.

#### What is XSS attacks?
Cross-Site Scripting (XSS) is a type of security vulnerability that affects web applications. XSS attacks occur when an attacker **injects malicious scripts into a web page** that is then viewed by other users. When a user visits the affected page, the malicious scripts are executed in their browser, allowing the attacker to steal sensitive information, such as login credentials, or perform other malicious actions on behalf of the user.

There are two main types of XSS attacks stored XSS and reflected XSS: 
 - `Stored XSS attacks` occur when the malicious script is stored on the server and served to every user who visits the affected page. 
 - `Reflected XSS attacks` occur when the malicious script is included in the URL and is reflected back to the user in the response.

XSS attacks can have serious consequences for both users and the web application. To prevent XSS attacks, web developers should carefully validate and sanitize user input and ensure that any dynamic content is properly escaped before it is included in the page. 

