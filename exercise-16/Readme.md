# **Recap Practice**

To push your boundaries, this practice project will require you to combine **everything** you've learned so far:

1. Primitive types & Arrays
2. Loops (`loop`, `while`, `for`) and branch evaluation (`if`, `if let`)
3. Stack vs. Heap allocation
4. Ownership moves & Borrowing rules (`&` vs `&mut`)
5. String (`String`) vs. Slices (`&str`) and range slicing `[start..end]`

---

## 🏗️ The Mini-Capstone: The API Request Log Analyzer

**Scenario:** You are building a high-performance telemetry engine for your backend server. You receive a series of raw HTTP request log strings. Your job is to process these logs, analyze their routing paths, count successful operations, and mask sensitive authorization headers using *zero allocation copies* (relying on `&str` views wherever possible).

### 📋 The Data Setup

Copy this exact array into the beginning of your `main` function:

```rust
let logs = [
    "GET /api/v1/users HTTP/1.1 | STATUS: 200 | AUTH: Bearer xyz123",
    "POST /api/v1/deploy HTTP/1.1 | STATUS: 201 | AUTH: Bearer abc456",
    "GET /api/v1/admin HTTP/1.1 | STATUS: 403 | AUTH: Bearer ttt789",
    "POST /api/v1/users HTTP/1.1 | STATUS: 500 | AUTH: Bearer qqq000",
];

```

---

### 🚀 Your Implementation Requirements

Write a single Rust file with the following components:

#### 1. The Masking Engine (`fn mask_auth`)

* Create a function named `mask_auth` with this signature:
```rust
fn mask_auth(raw_log: &str) -> String

```


* Inside this function, find where the token value starts (the string characters after `"AUTH: Bearer "`).
* Use **range slicing** to extract just the token string slice (`&str`) from the log.
* Create a new `String` variable that replaces the actual token characters with `"CLASSIFIED"`.
* *Hint:* You can construct the final returned `String` by combining a slice of the log up to the token point and appending your mask text.

#### 2. The Verification Engine (`fn is_successful`)

* Create a function named `is_successful` that takes a string slice reference to a log and returns a boolean (`bool`).
* It must check if the log contains `"STATUS: 200"` or `"STATUS: 201"`.

#### 3. The Orchestrator (`fn main`)

* Create a mutable counter on the stack: `let mut successful_requests = 0;`.
* Loop through the `logs` array using a **`for` loop**.
* Inside the loop, pass each log reference to `is_successful`. If it returns `true`, increment your counter.
* Inside the same loop, pass each log reference to `mask_auth` and print out the secure, masked log string it returns.
* After the loop finishes, print out the total number of successful requests.

---

### 🏁 Desired Output

Your program should compile cleanly with zero `.clone()` calls on your original strings and output exactly this:

```text
[SECURE LOG]: GET /api/v1/users HTTP/1.1 | STATUS: 200 | AUTH: Bearer CLASSIFIED
[SECURE LOG]: POST /api/v1/deploy HTTP/1.1 | STATUS: 201 | AUTH: Bearer CLASSIFIED
[SECURE LOG]: GET /api/v1/admin HTTP/1.1 | STATUS: 403 | AUTH: Bearer CLASSIFIED
[SECURE LOG]: POST /api/v1/users HTTP/1.1 | STATUS: 500 | AUTH: Bearer CLASSIFIED

Analysis Complete. Total Successful Requests (2xx): 2

```

---

Take your time writing this out in your editor. If the compiler flags an ownership issue, or if you get stuck on finding the right character index bounds for the string slicing, drop your draft here and we'll troubleshoot it step-by-step!