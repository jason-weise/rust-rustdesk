<p align="center">
  <img src="../res/logo-header.svg" alt="RustDesk - Your remote desktop"><br>
  <a href="#free-public-servers">Servers</a> •
  <a href="#raw-steps-to-build">Build</a> •
  <a href="#how-to-build-with-docker">Docker</a> •
  <a href="#file-structure">Structure</a> •
  <a href="#snapshot">Snapshot</a><br>
  [<a href="docs/README-CS.md">česky</a>] | [<a href="docs/README-ZH.md">中文</a>] | | [<a href="docs/README-HU.md">Magyar</a>] | [<a href="docs/README-ES.md">Español</a>] | [<a href="docs/README-FA.md">فارسی</a>] | [<a href="docs/README-FR.md">Français</a>] | [<a href="docs/README-DE.md">Deutsch</a>] | [<a href="docs/README-PL.md">Polski</a>] | [<a href="docs/README-ID.md">Indonesian</a>] | [<a href="docs/README-FI.md">Suomi</a>] | [<a href="docs/README-ML.md">മലയാളം</a>] | [<a href="docs/README-JP.md">日本語</a>] | [<a href="docs/README-NL.md">Nederlands</a>] | [<a href="docs/README-IT.md">Italiano</a>] | [<a href="docs/README-RU.md">Русский</a>] | [<a href="docs/README-PTBR.md">Português (Brasil)</a>] | [<a href="docs/README-EO.md">Esperanto</a>] | [<a href="docs/README-KR.md">한국어</a>] | [<a href="docs/README-AR.md">العربي</a>] | [<a href="docs/README-VN.md">Tiếng Việt</a>]<br>
  <b>  لغتك الأم,  <a href="https://github.com/rustdesk/doc.rustdesk.com">Doc</a> و <a href="https://github.com/rustdesk/rustdesk/tree/master/src/lang">RustDesk UI</a>, README نحن بحاجة إلى مساعدتك لترجمة هذا </b>
</p>

[Discord](https://discord.gg/nDceKgxnkV) | [Twitter](https://twitter.com/rustdesk) | [Reddit](https://www.reddit.com/r/rustdesk) :تواصل معنا عبر

[![ko-fi](https://ko-fi.com/img/githubbutton_sm.svg)](https://ko-fi.com/I2I04VU09)

.Rustبرنامج آخر لسطح المكتب عن بعد، مكتوب بـ
يعمل خارج الصندوق، لا حاجة إلى إعدادات. لديك سيطرة كاملة على بياناتك، دون مخاوف بشأن الأمن. يمكنك استخدام خادم
  الخاص بنا rendezvous/relay
[جهز لنفسك واحدا](https://rustdesk.com/server), أو
[خاص بك rendezvous/relay أكتب خادم](https://github.com/rustdesk/rustdesk-server-demo).

![image](https://user-images.githubusercontent.com/71636191/171661982-430285f0-2e12-4b1d-9957-4a58e375304d.png)

لمساعدتك على ذلك [`docs/CONTRIBUTING.md`](docs/CONTRIBUTING.md) يرحب بمساهمة الجميع. اطلع على  RustDesk.

[**؟ RustDesk كيفية يعمل**](https://github.com/rustdesk/rustdesk/wiki/How-does-RustDesk-work%3F)

[**BINARY تنزيل**](https://github.com/rustdesk/rustdesk/releases)

## خوادم مفتوحة ومجانية

فيما يلي الخوادم التي تستخدمها مجانًا، وقد تتغير طوال الوقت. إذا لم تكن قريبًا من أحد هؤلاء، فقد تكون شبكتك بطيئة.
| الموقع | المورد | المواصفات |
| --------- | ------------- | ------------------ |
| Seoul | AWS lightsail | 1 VCPU / 0.5GB RAM |
| Singapore | Vultr | 1 VCPU / 1GB RAM |
| Dallas | Vultr | 1 VCPU / 1GB RAM | |

## التبعيات

 لواجهة المستخدم الرسومية [sciter](https://sciter.com/) نسخة سطح المكتب تستخدم
 بنفسك sciter dynamic library عليك تحميل

[Windows](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.win/x64/sciter.dll) |
[Linux](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.lnx/x64/libsciter-gtk.so) |
[MacOS](https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.osx/libsciter.dylib)

 Sciter إلى Flutter سنقوم بترحيل نسخة سطح المكتب من .Flutter تستخدم إصدارات الهاتف المحمول.

## خطوات البناء

- C++ build env و Rust development env قم بإعداد

- بطريقة صحيحة `VCPKG_ROOT` env variable وأعد [vcpkg](https://github.com/microsoft/vcpkg) ثبت

  - Windows: `vcpkg install libvpx:x64-windows-static libyuv:x64-windows-static opus:x64-windows-static`
  - Linux/MacOS: `vcpkg install libvpx libyuv opus`

- run `cargo run`

## [البناء](https://rustdesk.com/docs/en/dev/build/)

## Linux

### Ubuntu 18 (Debian 10)

```sh
sudo apt install -y g++ gcc git curl wget nasm yasm libgtk-3-dev clang libxcb-randr0-dev libxdo-dev libxfixes-dev libxcb-shape0-dev libxcb-xfixes0-dev libasound2-dev libpulse-dev cmake
```

### Fedora 28 (CentOS 8)

```sh
sudo yum -y install gcc-c++ git curl wget nasm yasm gcc gtk3-devel clang libxcb-devel libxdo-devel libXfixes-devel pulseaudio-libs-devel cmake alsa-lib-devel
```

### Arch (Manjaro)

```sh
sudo pacman -Syu --needed unzip git cmake gcc curl wget yasm nasm zip make pkg-config clang gtk3 xdotool libxcb libxfixes alsa-lib pipewire
```


### vcpkg تثبيت

```sh
git clone https://github.com/microsoft/vcpkg
cd vcpkg
git checkout 2021.12.01
cd ..
vcpkg/bootstrap-vcpkg.sh
export VCPKG_ROOT=$HOME/vcpkg
vcpkg/vcpkg install libvpx libyuv opus
```

### Fix libvpx (For Fedora)

```sh
cd vcpkg/buildtrees/libvpx/src
cd *
./configure
sed -i 's/CFLAGS+=-I/CFLAGS+=-fPIC -I/g' Makefile
sed -i 's/CXXFLAGS+=-I/CXXFLAGS+=-fPIC -I/g' Makefile
make
cp libvpx.a $HOME/vcpkg/installed/x64-linux/lib/
cd
```

### البناء

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
git clone https://github.com/rustdesk/rustdesk
cd rustdesk
mkdir -p target/debug
wget https://raw.githubusercontent.com/c-smile/sciter-sdk/master/bin.lnx/x64/libsciter-gtk.so
mv libsciter-gtk.so target/debug
VCPKG_ROOT=$HOME/vcpkg cargo run
```

###  X11 (Xorg) إلى Wayland تغيير

افتراضية GNOME session  ك Xorg إتبع [هذه](https://docs.fedoraproject.org/en-US/quick-docs/configuring-xorg-as-default-gnome-session/) الخطوات لإعداد Wayland لا تدعم RustDesk

## Docker طريقة البناء باستخدام

ابدأ باستنساخ المستودع وبناء الكونتاينر:

```sh
git clone https://github.com/rustdesk/rustdesk
cd rustdesk
docker build -t "rustdesk-builder" .
```

ثم، في كل مرة تحتاج إلى بناء التطبيق، قم بتشغيل الأمر التالي:

```sh
docker run --rm -it -v $PWD:/home/user/rustdesk -v rustdesk-git-cache:/home/user/.cargo/git -v rustdesk-registry-cache:/home/user/.cargo/registry -e PUID="$(id -u)" -e PGID="$(id -g)" rustdesk-builder
```

لاحظ أن البناء الأول قد يستغرق وقتًا أطول قبل تخزين التبعيات، وسيكون البناء اللاحق أسرع. بالإضافة إلى ذلك، إذا كنت بحاجة إلى تحديد وسائط مختلفة لأمر البناء، فيمكنك القيام بذلك في نهاية الأمر بوضع
`<OPTIONAL-ARGS>`
على سبيل المثال، إذا كنت ترغب في بناء إصدار محسن، فستقوم بتشغيل الأمر أعلاه متبوعًا بـ
`--release`
:سيكون الملف القابل للتنفيذ الناتج متاحًا في مجلد تارغت، ويمكن تشغيله باستخدام

```sh
target/debug/rustdesk
```

:أو في حال قمت ببناء إصدار محسن

```sh
target/release/rustdesk
```

RustDesk يرجى التأكد من أنك تنفذ هذه الأوامر من جذر مستودع
وإلا فقد لا يتمكن التطبيق من العثور على الموارد المطلوبة. لاحظ أيضًا أن الأوامر الفرعية الأخرى مثل
`install` أو `run`
لا يتم دعمها حاليًا عبر هذه الطريقة لأنها ستقوم بتثبيت أو تشغيل البرنامج داخل الكونتاينر بدلاً من الهوست.

## هيكل الملف

- **[libs/hbb_common](https://github.com/rustdesk/rustdesk/tree/master/libs/hbb_common)**: وظائف  لنقل الملفات، وبعض وظائف المرافق الأخرى tcp/udp، protobuf ترميز الفيديو، إعدادات

- **[libs/scrap](https://github.com/rustdesk/rustdesk/tree/master/libs/scrap)**: التقاط الشاشة
- **[libs/enigo](https://github.com/rustdesk/rustdesk/tree/master/libs/enigo)**: التحكم في لوحة المفاتيح/الماوس الخاصة بكل منصة
- **[src/ui](https://github.com/rustdesk/rustdesk/tree/master/src/ui)**: واجهة المستخدم الرسومية
- **[src/server](https://github.com/rustdesk/rustdesk/tree/master/src/server)**: خدمات الصوت/الحافظة/المدخلات/الفيديو، ووصلات الشبكة
- **[src/client.rs](https://github.com/rustdesk/rustdesk/tree/master/src/client.rs)**: بدء اتصال متقارن
- **[src/rendezvous_mediator.rs](https://github.com/rustdesk/rustdesk/tree/master/src/rendezvous_mediator.rs)**: أو المنقول عن بُعد (TCP hole punching) انتظر الاتصال المباشر [rustdesk-server](https://github.com/rustdesk/rustdesk-server) الإتصال ب
- **[src/platform](https://github.com/rustdesk/rustdesk/tree/master/src/platform)**: رمز خاص بكل منصة
- **[flutter](https://github.com/rustdesk/rustdesk/tree/master/flutter)**: رمز الهاتف المحمول
- **[flutter/web/js](https://github.com/rustdesk/rustdesk/tree/master/flutter/web/js)**:Flutter  لعميل الويب الخاص ب Javascript

## لقطات

![image](https://user-images.githubusercontent.com/71636191/113112362-ae4deb80-923b-11eb-957d-ff88daad4f06.png)

![image](https://user-images.githubusercontent.com/71636191/113112619-f705a480-923b-11eb-911d-97e984ef52b6.png)

![image](https://user-images.githubusercontent.com/71636191/113112857-3fbd5d80-923c-11eb-9836-768325faf906.png)

![image](https://user-images.githubusercontent.com/71636191/135385039-38fdbd72-379a-422d-b97f-33df71fb1cec.png)
