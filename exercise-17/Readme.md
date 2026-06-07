# **Recap Practice part 2**

This time, we are going to simulate a real-world backend component: **An API Gateway Rate-Limiter & IP Parser**.

This project requires you to combine loops, conditional branch logic, strict memory boundaries (`&mut`), string matching, and explicit range slicing *without* manual type casting or using `.clone()` on the original data.

---

## 🏗️ The Mini-Capstone: The API Gateway IP & Rate-Limit Analyzer

**Scenario:** Your backend service is under a minor traffic spike. You receive a raw dump of incoming network traffic packets as an array of string slices. Your job is to parse each packet to extract the client's IP address, determine if they are hitting a restricted administrative route, and block any requests that violate your rate limits.

### 📋 The Data Setup

Copy this array into your `main` function:

```rust
let traffic_packets = [
    "IP: 192.168.1.50  | ROUTE: /api/v1/data  | REQ_COUNT: 04",
    "IP: 10.0.0.12     | ROUTE: /api/v1/admin | REQ_COUNT: 15",
    "IP: 172.16.254.1  | ROUTE: /api/v1/users | REQ_COUNT: 02",
    "IP: 192.168.1.99  | ROUTE: /api/v1/admin | REQ_COUNT: 07",
];

```

---

### 🚀 Your Implementation Requirements

Write a single Rust file containing the following components:

#### 1. The Route Inspector (`fn is_admin_route`)

* Create a function named `is_admin_route` that takes a string slice parameter (`packet: &str`) and returns a `bool`.
* It should check if the packet contains the path `"/api/v1/admin"`.

#### 2. The IP Extractor & Sanitizer (`fn extract_and_format_ip`)

* Create a function named `extract_and_format_ip` with this signature:
```rust
fn extract_and_format_ip(packet: &str) -> String

```


* Inside all packets, the IP address characters always start at index **`4`** and end right before the first vertical pipe character (at index **`18`**).
* Use **range slicing** (`[start..end]`) to isolate just the IP string slice from the packet.
* Clean up any trailing spaces by calling `.trim()` on your extracted slice.
* Construct and return a new `String` that prefixes the IP with a tag, matching this layout: `"CLIENT_IP: [the_isolated_ip]"`.

#### 3. The Orchestrator (`fn main`)

* Create a mutable counter on the stack to keep track of security flags: `let mut admin_route_attempts = 0;`.
* Loop through the `traffic_packets` array using a **`for` loop**.
* Inside the loop, extract and print the formatted IP for every single packet by calling `extract_and_format_ip`.
* Use your `is_admin_route` function to check if the current packet is trying to access the admin portal. If it returns `true`:
* Increment your `admin_route_attempts` counter.
* Use range slicing or string matching to check the `REQ_COUNT` at the very end of the string. If the request count is greater than `"10"` (you can do a simple string slice check like `packet.contains("REQ_COUNT: 15")` or check the text slice bounds `&packet[47..]`), print out an explicit **`[BLOCK]`** warning message. Otherwise, print an **`[ALLOW]`** log.


* After the loop terminates, print out the final total count of administrative access attempts.

---

### 🏁 Desired Output

Your program should compile with zero allocation shortcuts and produce this clean backend log:

```text
[LOG] CLIENT_IP: 192.168.1.50
[LOG] CLIENT_IP: 10.0.0.12
[SECURITY ALERT] Admin access requested! Status: [BLOCK] (Rate Limit Exceeded)
[LOG] CLIENT_IP: 172.16.254.1
[LOG] CLIENT_IP: 192.168.1.99
[SECURITY ALERT] Admin access requested! Status: [ALLOW] (Within Normal Limits)

Total Admin Route Access Requests Monitored: 2

```

---

### 💡 Hint to protect you from the Compiler

Remember that `extract_and_format_ip` returns a brand new owned `String`. When you call it inside your loop, print it out immediately so its lifetime doesn't conflict with any other references!

Fire up your editor, give this architecture a spin, and show me your code layout when you're ready!