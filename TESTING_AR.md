# كيف تجرب StreamBit بنفسك - دليل المبتدئين

## الخطوة 1️⃣: افتح PowerShell

1. اضغط `Win + X`
2. اختر **Windows PowerShell** أو **Terminal**
3. أو ابحث عن "PowerShell" في قائمة Start

---

## الخطوة 2️⃣: انتقل لمجلد المشروع

```powershell
cd C:\MAMP\htdocs\flowxtra\streambit
```

**للتأكد أنك في المكان الصحيح:**
```powershell
ls
```
يجب أن تشوف ملفات مثل: `Cargo.toml`, `README.md`, `streambit-core`, `streambit-vision`

---

## الخطوة 3️⃣: إضافة Cargo للـ PATH (مهم!)

**نسخ والصق هذا الأمر:**
```powershell
$env:Path += ";$env:USERPROFILE\.cargo\bin"
```

**للتأكد أن Cargo يشتغل:**
```powershell
cargo --version
```
يجب أن تشوف رقم الإصدار مثل: `cargo 1.87.0`

---

## الخطوة 4️⃣: تشغيل المثال

### الطريقة الأولى (Debug - أبطأ لكن أسرع في البناء):
```powershell
cargo run --package streambit-vision --example image_processing_demo
```

### الطريقة الثانية (Release - أسرع بكثير!):
```powershell
cargo run --release --package streambit-vision --example image_processing_demo
```

**ملاحظة:** أول مرة راح ياخذ وقت (1-2 دقيقة) لأنه يحمل المكتبات ويبني المشروع.

---

## الخطوة 5️⃣: شاهد النتائج! 🎉

راح تشوف شيء مثل هذا:

```
🚀 StreamBit Vision - Image Processing Demo

============================================================
✅ Image Processor created
   - Resize Mode: Bilinear
   - Threads: Auto (all cores)

📝 Demo 1: Creating test images...
   ✅ Created 3 test images (red, green, blue)

📝 Demo 2: Loading single image...
   ✅ Loaded successfully!
   - Shape: [3, 224, 224]
   - Elements: 150528

📝 Demo 3: Batch loading (parallel)...
   ✅ Batch loaded successfully!
   - Images: 3
   - Time: 34.16ms
   - Throughput: 88 images/sec

... إلخ
```

---

## 🎨 تجربة مع صورك الخاصة

### 1. ضع صورك في مجلد:
```powershell
mkdir my_images
# ثم انسخ صورك (JPG, PNG) إلى my_images
```

### 2. عدل الكود:
افتح الملف: `streambit-vision\examples\image_processing_demo.rs`

ابحث عن السطر:
```rust
let image_paths = vec![
    "test_images/red.png",
    "test_images/green.png",
    "test_images/blue.png",
];
```

غيره إلى:
```rust
let image_paths = vec![
    "my_images/photo1.jpg",
    "my_images/photo2.jpg",
    "my_images/photo3.jpg",
];
```

### 3. شغل مرة ثانية:
```powershell
cargo run --package streambit-vision --example image_processing_demo
```

---

## 🔧 حل المشاكل الشائعة

### ❌ "cargo: command not found"
**الحل:**
```powershell
$env:Path += ";$env:USERPROFILE\.cargo\bin"
```
أو أعد تشغيل PowerShell

### ❌ "Failed to load image"
**الحل:**
- تأكد أن الصور موجودة في المجلد الصحيح
- تأكد من اسم الملف صحيح (مع الامتداد .jpg أو .png)
- جرب مع الصور التجريبية أولاً

### ❌ البناء بطيء جداً
**الحل:**
- هذا طبيعي في أول مرة (يحمل كل المكتبات)
- المرات القادمة راح تكون أسرع بكثير
- استخدم `--release` للأداء الأقصى

---

## 📊 فهم النتائج

### Shape: [3, 224, 224]
- **3** = عدد القنوات (Red, Green, Blue)
- **224** = العرض بالبكسل
- **224** = الارتفاع بالبكسل

### Throughput: 88 images/sec
- عدد الصور المعالجة في الثانية
- في **Debug mode** = ~88 صورة/ثانية
- في **Release mode** = ~400-800 صورة/ثانية (أسرع 5-10 مرات!)

### Time: 34.16ms
- الوقت المستغرق لمعالجة الدفعة
- كل ما كان أقل = أفضل

---

## 🚀 تجارب إضافية

### تجربة 1: زيادة عدد الصور
عدل الكود ليحمل 10 أو 20 صورة بدل 3

### تجربة 2: تغيير حجم الصورة
غير `(224, 224)` إلى `(512, 512)` أو `(128, 128)`

### تجربة 3: تجربة أوضاع Resize المختلفة
- `ResizeMode::Nearest` - الأسرع (جودة أقل)
- `ResizeMode::Bilinear` - متوازن
- `ResizeMode::Bicubic` - جودة أفضل
- `ResizeMode::Lanczos3` - أفضل جودة (الأبطأ)

---

## 📁 الملفات المهمة

```
streambit/
├── streambit-vision/
│   ├── examples/
│   │   └── image_processing_demo.rs  ← المثال الرئيسي (عدل هنا)
│   └── src/
│       └── image_proc.rs              ← كود معالجة الصور
├── test_images/                       ← الصور التجريبية (تنشأ تلقائياً)
│   ├── red.png
│   ├── green.png
│   └── blue.png
└── TESTING.md                         ← دليل الاختبار
```

---

## 💡 نصائح

1. **استخدم Release mode للأداء الحقيقي:**
   ```powershell
   cargo run --release --package streambit-vision --example image_processing_demo
   ```

2. **جرب مع صور كبيرة (1000+) لترى قوة المعالجة المتوازية**

3. **راقب استخدام CPU - يجب أن يكون قريب من 100% على كل الأنوية**

4. **قارن الأداء مع مكتبات Python (OpenCV, Pillow)**

---

## ❓ أسئلة شائعة

**س: كم يستغرق البناء أول مرة؟**
ج: 1-3 دقائق (يحمل كل المكتبات)

**س: هل أحتاج إنترنت؟**
ج: فقط في أول مرة لتحميل المكتبات

**س: كيف أوقف البرنامج؟**
ج: اضغط `Ctrl + C` في PowerShell

**س: كيف أمسح الملفات المؤقتة؟**
ج: `cargo clean` (لكن راح يحتاج يبني من جديد)

---

**جاهز للتجربة؟** 🚀

افتح PowerShell وابدأ من الخطوة 2!
