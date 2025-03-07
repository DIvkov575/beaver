# 🦫 Beaver SIEM

*Secure. Analyze. Monitor.*

Beaver SIEM is a cutting-edge **data security log analysis tool** designed to protect your infrastructure by monitoring and analyzing logs in real-time. It ensures your system’s safety by helping identify potential security breaches and threats through advanced log parsing and analysis techniques.

---

## 🚀 **Build**

To get started with Beaver SIEM, you will need to install and build it using the standard tools. Make sure to set up the required environment before proceeding.
```
Cargo install --release --path .
```
⚙️ *This step will build Beaver SIEM in release mode. Reload shell after install*

---

## 🛠️ **Setup**

After building, you can initialize Beaver SIEM through a simple setup process that creates the necessary configurations and sets up your environment.
```
Beaver init
```
⚙️ *Ensure that your system has the required permissions and environment variables configured.*


---

### 📋 **Current Todo:**

> The following tasks are in progress or planned for the next release:

- **l_0** - crj shutoff - missing `{}` as second arg in the last `.get()` chain method call
- **l_1** - Add SA delegation - destructured log -> BQ - logging

---

### 💡 **Ideas for Future Enhancements:**

- Disable `detections_gen.py` regeneration to streamline the development process.
- Implement batching to improve efficiency in BigQuery by deduplicating writes after log destructuring.
  
    📝 *This will help reduce the amount of redundant data being written, enhancing write efficiency.*

---

### ⏳ **Current Progress:**

- Create Cloud Run jobs.
- Set up BigQuery.
- Create Pub/Sub topic **(1)** and subscription **(2)** for BigQuery integration.
- Provision storage bucket for log storage.
- Link Cloud Run jobs to storage bucket for secure storage and retrieval.

---


> “Stay secure, stay informed.”
