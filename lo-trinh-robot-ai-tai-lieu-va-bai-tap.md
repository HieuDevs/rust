# Lộ trình tự học robot AI — Tài liệu, bài tập và tiêu chí hoàn thành

**Phiên bản:** 10/10/2026  
**Đối tượng:** mới học điện tử và robot; tự học Rust riêng.  
**Dự án xuyên suốt:** robot để bàn dùng ESP32-S3, màn hình mắt, một servo vị trí, nút bấm để nói, micro và loa; laptop làm máy chủ gọi dịch vụ AI qua mạng.  
**Cách tổ chức:** 33 đơn vị học theo tuần; 198 câu hỏi lý thuyết, 66 bài tập tính toán/thiết kế có đáp án đối chiếu và 132 buổi thực hành theo trình tự.

> Đây là chương trình tự học có giao bài. Phần “định hướng bản chất” giúp bạn biết cần tìm điều gì trong sách; phần đó không thay thế việc đọc bài học được chỉ định và tự giải bài tập. Mục tiêu là học đủ để tự giải thích, đo, tìm lỗi và thay đổi thiết kế.

## Mục lục

- [Cách dùng tài liệu](#cach-dung)
- [Bộ tài liệu chính và cách đọc](#tai-lieu)
- [Chuẩn bị và kiến thức đầu vào](#chuan-bi)
- [Tổng quan các giai đoạn](#giai-doan)
- [Kế hoạch từng tuần](#ke-hoach)
  - [Tuần 01 — Điện tích điện áp dòng điện và năng lượng](#tuan-01)
  - [Tuần 02 — Đo lường là một phần của mạch](#tuan-02)
  - [Tuần 03 — LED không phải điện trở](#tuan-03)
  - [Tuần 04 — Nút điện và bảo toàn dòng](#tuan-04)
  - [Tuần 05 — Chia áp tải và đầu vào thả nổi](#tuan-05)
  - [Tuần 06 — Tụ điện thời gian và mô hình gần đúng](#tuan-06)
  - [Tuần 07 — Chương trình trở thành hành vi phần cứng như thế nào](#tuan-07)
  - [Tuần 08 — GPIO là mạch điện điều khiển được](#tuan-08)
  - [Tuần 09 — Từ tiếp điểm cơ khí đến một sự kiện nhấn](#tuan-09)
  - [Tuần 10 — ADC lấy mẫu và PWM điều chế thời gian](#tuan-10)
  - [Tuần 11 — I2C và ý nghĩa của một lần truyền thành công](#tuan-11)
  - [Tuần 12 — Hình ảnh bộ nhớ và thời gian](#tuan-12)
  - [Tuần 13 — Servo là một hệ điều khiển có phản hồi bên trong](#tuan-13)
  - [Tuần 14 — Mô men trọng tâm và quán tính](#tuan-14)
  - [Tuần 15 — Nguồn có trở kháng và tải thay đổi theo thời gian](#tuan-15)
  - [Tuần 16 — Máy trạng thái làm hành vi có thể lý giải](#tuan-16)
  - [Tuần 17 — Đồng thời hàng đợi và giới hạn tài nguyên](#tuan-17)
  - [Tuần 18 — Mạng là nhiều lớp độc lập](#tuan-18)
  - [Tuần 19 — Giao thức là một thỏa thuận về ý nghĩa](#tuan-19)
  - [Tuần 20 — Timeout retry và tác động lặp](#tuan-20)
  - [Tuần 21 — Mô hình ngôn ngữ và đầu vào thực tế của nó](#tuan-21)
  - [Tuần 22 — Dữ liệu bền vững và định danh](#tuan-22)
  - [Tuần 23 — Bộ nhớ AI là quy trình chọn thông tin](#tuan-23)
  - [Tuần 24 — Âm thanh là tín hiệu được lấy mẫu](#tuan-24)
  - [Tuần 25 — I2S DMA và bộ đệm phát](#tuan-25)
  - [Tuần 26 — Diễn giải đúng dữ liệu từ micro](#tuan-26)
  - [Tuần 27 — Chuỗi nhận dạng hội thoại và tổng hợp tiếng nói](#tuan-27)
  - [Tuần 28 — Đo độ trễ theo ranh giới rõ ràng](#tuan-28)
  - [Tuần 29 — Ngân sách bộ nhớ và ổn định theo thời gian](#tuan-29)
  - [Tuần 30 — Kiểm thử giả thuyết và khả năng phục hồi](#tuan-30)
  - [Tuần 31 — Vỏ là một phần của hệ thống](#tuan-31)
  - [Tuần 32 — Kết nối điện cũng là kết cấu cơ khí](#tuan-32)
  - [Tuần 33 — Tái lập sao lưu và ranh giới bản đầu](#tuan-33)
- [Các mốc nghiệm thu](#nghiem-thu)
- [Mẫu ghi chép và báo cáo thực nghiệm](#mau)
- [Đáp án và hướng kiểm tra 66 bài tập](#dap-an)
- [Từ vựng cần hiểu](#thuat-ngu)

<a id="cach-dung"></a>
## 1. Cách dùng tài liệu

### 1.1. Bạn học ở đâu và làm gì trước?

Với mỗi tuần, làm lần lượt:

1. **Đọc bảng tài liệu bắt buộc.** Mở nguồn số 1, đọc đúng phần được giao, làm lại ví dụ trên giấy. Sau đó mới đọc nguồn tiếp theo.
2. **Viết ghi chú của riêng bạn.** Mỗi khái niệm có định nghĩa, đơn vị nếu có, sơ đồ và ví dụ của robot. Không chép nguyên đoạn từ sách.
3. **Trả lời sáu câu lý thuyết.** Với câu “vì sao”, phải viết quan hệ nguyên nhân–kết quả; một định nghĩa hoặc một từ khóa chưa đủ.
4. **Làm hai bài tập có mã.** Tự trình bày giả thiết, công thức/mô hình, phép tính hoặc thiết kế, rồi kết luận. Đáp án đặt ở cuối để không đọc trước.
5. **Làm bốn phần thực hành theo thứ tự.** Đây là các phần việc, có thể chia thành nhiều buổi nếu cần. Viết dự đoán trước khi đo/chạy.
6. **Làm biến thể cuối tuần.** Thay một điều kiện để kiểm tra bạn có hiểu mô hình hay chỉ làm đúng bài mẫu.
7. **Hoàn thành hồ sơ và tiêu chí qua tuần.** Nếu chưa đạt, đọc lại đúng phần còn thiếu và thử lại; không chạy theo số tuần.

### 1.2. Phân bổ thời gian

Dự kiến khoảng **10–14 giờ mỗi đơn vị tuần**, gồm 3–4 giờ đọc và ghi chú, 2–3 giờ giải bài lý thuyết, 4–6 giờ thực hành và khoảng 1 giờ tổng kết. Thời gian Rust không tính trong số này. Nếu chỉ có 5–7 giờ/tuần, có thể dành hai tuần lịch cho một đơn vị. Các phần âm thanh/tích hợp thường cần thêm thời gian tìm lỗi.

Không cần học xong mọi môn rồi mới làm robot: đọc và thực hành song song. Cũng không bỏ phần đọc chỉ vì code mẫu chạy. Khi vướng cú pháp/toolchain Rust, tách vấn đề đó khỏi bài toán điện tử để biết mình đang bị chặn ở lớp nào.

### 1.3. Cách tự chấm mức hiểu

| Mức | Bạn làm được gì? | Có đủ để chuyển tuần? |
|---|---|---|
| 1 | Nhắc lại tên và định nghĩa | Chưa |
| 2 | Giải thích được ví dụ đã đọc | Chưa nếu chưa tự làm |
| 3 | Dự đoán đúng một biến thể và đo để kiểm tra | Đạt nền tảng |
| 4 | Khi sai dự đoán, đặt giả thuyết và thiết kế phép thử phân biệt nguyên nhân | Mục tiêu của lộ trình |

Không đặt ngưỡng sai số chung như “mọi phép đo phải lệch dưới 5%”. Sai lệch chấp nhận được tùy dung sai linh kiện, thiết bị đo và mô hình. Điều bắt buộc là ghi số thật, điều kiện và lời giải thích có bằng chứng.

### 1.4. Những gì thuộc và không thuộc phạm vi

- **Trong phạm vi:** điện DC, linh kiện cơ bản, MCU, GPIO/ADC/PWM/bus, servo/nguồn/cơ khí nhẹ, tổ chức firmware, mạng, dữ liệu, API AI, âm thanh và tích hợp.
- **Rust:** bạn tự học. Từ tuần 7 phải tự viết/nạp được chương trình tương ứng; tài liệu chỉ giao hành vi và bài kiểm tra, không dạy lại cú pháp.
- **Bản đầu chưa cần:** ROS 2, robot bánh xe, động học nhiều khớp, huấn luyện mô hình, AI offline, camera, PCB riêng, pin và hội thoại nghe–nói đồng thời.
- **Không mặc định chốt GPIO hoặc nguồn chỉ từ ví dụ:** phải tra đúng bo/module thực dùng. Tên chip/biến thể, định mức và phiên bản tài liệu là một phần của bài học.

<a id="tai-lieu"></a>
## 2. Bộ tài liệu chính và cách đọc

### 2.1. Dùng ít nguồn nhưng đọc đúng phần

| Nhóm | Nguồn học chính | Cách sử dụng |
|---|---|---|
| Điện tử | [All About Circuits — Direct Current](https://www.allaboutcircuits.com/textbook/direct-current/) | Sách nền tảng cho tuần 1–6; đi theo bài/chương chỉ định, tự làm lại ví dụ. Mục lục có cả các tập bán dẫn và mạch số. |
| Vật lý sâu hơn | [OpenStax — University Physics Volume 2](https://openstax.org/books/university-physics-volume-2/pages/9-1-electrical-current) | Đọc định nghĩa và lập luận khi chưa hiểu bản chất V, I, năng lượng hoặc điện dung. Phần giải tích là đào sâu, không chặn bài mạch đầu tiên. |
| Lắp/đo | [Adafruit — Breadboards](https://learn.adafruit.com/breadboards-for-beginners/breadboards) và [Adafruit — Multimeters](https://learn.adafruit.com/multimeters) | Đọc trước khi cắm mạch hoặc đặt que đo; đối chiếu hướng dẫn đồng hồ thực mua. |
| Bo thực dùng | [MakerEdu — MKE-K01](https://github.com/makereduvn/MKE-K01-ESP32-S3-DEV-KIT) | Chỉ áp cho MKE-K01 đúng phiên bản. Nếu bo khác, thay bằng schematic/pinout của bo đó. |
| Vi điều khiển | [ESP32-S3 GPIO](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/peripherals/gpio.html) | Tài liệu tra cấu hình và giới hạn của ESP32-S3; đọc phần giải thích trước API. Mỗi tuần dẫn tới ngoại vi liên quan. |
| Tổ chức công việc | [Adafruit — Multi-tasking Part 1](https://learn.adafruit.com/multi-tasking-the-arduino-part-1/a-classy-solution) và [ESP-IDF FreeRTOS](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/system/freertos_idf.html) | Bài đầu giúp hình dung; tài liệu RTOS giải thích task, queue và lập lịch sâu hơn. |
| Mạng | [MDN — How does the Internet work?](https://developer.mozilla.org/en-US/docs/Learn_web_development/Howto/Web_mechanics/How_does_the_Internet_work) và [MDN — HTTP Overview](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Overview) | Đọc ví dụ request/response, sau đó đối chiếu log thật. |
| Cơ sở dữ liệu | [SQLBolt — Bài học SQL có bài tập tương tác](https://sqlbolt.com/) và [SQLite — SQL Language](https://www.sqlite.org/lang.html) | Học bằng bài tập tương tác trước; dùng tài liệu SQLite để tra chính xác khi xây dữ liệu. |
| AI API | [Gemini — Text generation](https://ai.google.dev/gemini-api/docs/text-generation) | Nguồn mẫu để hiểu input/context/output. Có thể chọn nhà cung cấp khác nhưng phải làm cùng bài đánh giá. |
| Âm thanh | [Audacity — Digital Audio Fundamentals](https://manual.audacityteam.org/man/digital_audio.html), [ESP32-S3 I²S](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/peripherals/i2s.html), [TDK — INMP441 Datasheet](https://product.tdk.com/system/files/dam/doc/product/sw_piezo/mic/mems-mic/data_sheet/inmp441.pdf) | Đi từ tín hiệu/file trên laptop đến cấu hình bus và micro; không bắt đầu ngay bằng pipeline AI. |

### 2.2. Khi mở một trang tiếng Anh

Đọc tiêu đề, hình và ví dụ trước. Dịch nội dung sang tiếng Việt nếu cần nhưng giữ tên chân, đơn vị, công thức, mã linh kiện và tên API. Tạo bảng thuật ngữ cá nhân: từ tiếng Anh → nghĩa trong mạch của bạn → ví dụ → chỗ dễ nhầm.

Với sách: ghi lại lập luận và tự suy ra công thức. Với datasheet: tìm chức năng, pinout, điều kiện hoạt động, giới hạn tuyệt đối, timing và sơ đồ ứng dụng. Với tài liệu API: tìm input/output, quyền sở hữu tài nguyên, điều kiện lỗi và vòng đời.

Tài liệu Arduino được dùng để học nguyên lý, không yêu cầu chuyển ngôn ngữ. Đừng bê chân/điện áp Arduino sang ESP32. Liên kết ESP-IDF có `stable` là điểm vào tra cứu; chọn phiên bản khớp nền tảng Rust của dự án trước khi dùng hàm/cấu hình. Một ví dụ dùng `delay` trên nền tảng đơn giản không có nghĩa mọi lệnh sleep đều chiếm CPU trên mọi RTOS; phải xét task và runtime cụ thể.

Nếu trang thay đổi cấu trúc, tìm **tên bài hoặc tên mục đã ghi trong bảng tuần** trong chính trang/mục lục của nguồn đó. Không đổi sang tài liệu chip khác chỉ vì tìm dễ hơn.

<a id="chuan-bi"></a>
## 3. Chuẩn bị và kiến thức đầu vào

### 3.1. Toán cần dùng

Trước tuần 1, tự đổi được: 3 mA = 0,003 A; 4,7 kΩ = 4700 Ω; 100 µF = 0,0001 F; 250 ms = 0,25 s; 1 byte = 8 bit. Biết giải phương trình đơn giản, phần trăm và vẽ bảng/đồ thị.

Tuần 6 học thêm hàm mũ khi xem RC. Tuần 14 học mô-men và khoảng cách vuông góc. Tuần 28 dùng trung bình, trung vị và phạm vi. Không cần học xong giải tích trước mới được bắt đầu.

### 3.2. Mua theo lúc cần

| Trước tuần | Vật tư |
|---|---|
| 2 | Đồng hồ có hướng dẫn, breadboard, dây, hộp 2 pin AA có công tắc, pin, LED đỏ, nút; điện trở 330 Ω, 1 kΩ, 10 kΩ với nhiều chiếc mỗi loại |
| 5–6 | Biến trở 10 kΩ, điện trở 100 kΩ, tụ 100 µF và 100 nF có định mức phù hợp; linh kiện bán dẫn chỉ mua khi có bài lắp được kiểm tra |
| 7 | ESP32-S3 đúng biến thể, cáp USB truyền dữ liệu, dây hợp header |
| 11 | OLED I²C đúng chip điều khiển, điện áp và mức logic |
| 13 | Servo vị trí, nguồn phù hợp, đầu nối/dây chịu tải, tay/giá servo và vật liệu đầu nhẹ |
| 25–26 | MAX98357A, loa đúng trở kháng, INMP441, dây ngắn và đầu nối |
| 31–32 | Vật liệu vỏ thử, ốc/đế, đầu nối, vật tư hàn nếu tự làm |

Chưa cần mua oscilloscope. Logic analyzer có ích ở tuần 11 nhưng là tùy chọn; nếu chưa có, phải nhận thức giới hạn của DMM khi kiểm tra tín hiệu nhanh.

### 3.3. Quy tắc cho đúng các bài thực hành này

Dùng nguồn điện áp thấp phù hợp. Ngắt nguồn trước đổi dây; đo điện trở/thông mạch khi mạch không có điện. Không mắc đồng hồ ở chế độ dòng trực tiếp qua hai cực nguồn. Không cấp motor từ GPIO/rail 3,3 V của bo. Không cố giữ servo kẹt để thử lỗi. Kiểm tra cực tụ và nguồn/tín hiệu của từng module trước cấp điện.

<a id="giai-doan"></a>
## 4. Tổng quan các giai đoạn

| Tuần | Học phần | Năng lực phải đạt |
|---|---|---|
| 1–6 | Điện DC, đo lường, linh kiện | Lắp mạch từ sơ đồ, tự tính và đo được dòng/áp, hiểu ảnh hưởng tải và thời gian RC. |
| 7–12 | Vi điều khiển và giao tiếp | LED/nút/ADC/PWM/OLED chạy riêng và kết hợp, đọc được giới hạn chân và bus. |
| 13–17 | Chuyển động, nguồn và điều phối | Đầu nhẹ quay ổn, nguồn có sơ đồ, trạng thái/hủy/hàng đợi có quy tắc. |
| 18–20 | Mạng và hợp đồng giao tiếp | Robot gọi server, xử lý dữ liệu sai, timeout, yêu cầu trùng và mất mạng. |
| 21–23 | AI hội thoại và dữ liệu người dùng | Chat chữ, dữ liệu đúng người, xem/sửa/quên được kiểm chứng. |
| 24–27 | Âm thanh số và giọng nói | Tính dung lượng, thu/phát đúng format, hoàn thành lượt STT–AI–TTS. |
| 28–30 | Đo chất lượng và tìm lỗi | Bảng độ trễ/bộ nhớ, kiểm thử lỗi và bằng chứng sửa. |
| 31–33 | Vỏ, kết nối, bản hoàn thiện | Vỏ không phá âm/cơ; dây chắc; tài liệu tái lập và backup đã khôi phục. |

<a id="ke-hoach"></a>
## 5. Kế hoạch và bài tập từng tuần

**Quy ước:** sáu câu hỏi ký hiệu LT; hai bài có mã Txx-B1/B2, đáp án ở phần cuối; bốn phần thực hành TH1–TH4. “Đạt” có nghĩa bạn tự giải thích và kiểm chứng được, không chỉ có ảnh thiết bị chạy.


<a id="tuan-01"></a>
## Tuần 01 — Điện tích điện áp dòng điện và năng lượng

### A. Tiên quyết và mục tiêu

Bắt đầu ở đây. Cần đổi đơn vị và giải phương trình đơn giản; chưa cần bo hoặc code.

**Mục tiêu cuối tuần:** Trả lời được 6 câu bằng lời riêng; giải đúng cả hai bài có đơn vị; giải thích được vì sao một mạch hở vẫn có thể có điện áp.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [Voltage and Current](https://www.allaboutcircuits.com/textbook/direct-current/chpt-1/voltage-current/) | Đọc toàn bài; dừng lại ở mỗi sơ đồ để xác định hai điểm đo điện áp và đường dòng điện. | Tự vẽ lại mạch kín và mạch hở. |
| 2 | [Ohm’s Law](https://www.allaboutcircuits.com/textbook/direct-current/chpt-2/voltage-current-resistance-relate/) | Đọc quan hệ V–I–R và các ví dụ số; tự làm lại trước khi xem kết quả. | Một trang công thức có giải nghĩa đơn vị. |
| 3 | [OpenStax — University Physics Volume 2](https://openstax.org/books/university-physics-volume-2/pages/9-1-electrical-current) | Đọc mục Electrical Current ở mức định nghĩa; phần giải tích chỉ tham khảo. | Giải thích 1 A = 1 C/s và phân biệt điện tích với năng lượng. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** nguồn điện thật sự cung cấp gì, và vì sao có điện áp chưa chắc có dòng?

**Bản chất cần hiểu.** Điện tích là đại lượng vật lý gắn với hạt, đo bằng coulomb (C). Trong dây kim loại có sẵn electron dẫn. Pin dùng năng lượng hóa học để duy trì chênh lệch điện thế giữa hai cực. Nó không cần sản xuất electron mới cho từng lượt dòng điện.

Điện áp giữa A và B là chênh lệch điện thế, tương ứng chênh lệch thế năng trên một đơn vị điện tích: 1 V = 1 J/C. Khi tải nhận năng lượng, độ lớn năng lượng điện chuyển giao cho lượng điện tích Q qua chênh lệch điện áp V là E = V × Q trong trường hợp điện áp không đổi. Điện áp luôn là quan hệ giữa hai điểm. GND là mốc được chọn trong mạch, chưa có nghĩa nối đất bảo vệ.

Dòng điện là lượng điện tích ròng đi qua một tiết diện mỗi đơn vị thời gian: I = ΔQ/Δt; 1 A = 1 C/s. Điện áp mô tả chênh lệch năng lượng trên điện tích; dòng mô tả tốc độ chuyển điện tích. Có thể có điện áp giữa hai đầu hở nhưng không có dòng duy trì qua đường hở trong mô hình DC đơn giản.

Điện trở lý tưởng tuân theo V = I × R. Nguồn áp và tải cùng quyết định dòng trong mạch. Dòng qua tải không bị “ăn mất”; trong mạch nối tiếp ổn định, dòng trước và sau tải bằng nhau. Tải nhận năng lượng và biến đổi nó thành nhiệt, ánh sáng hoặc chuyển động.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T01-LT1:** Điện tích, dòng điện, điện áp và năng lượng khác nhau thế nào? Với mỗi đại lượng, nêu ký hiệu, đơn vị và một câu mô tả không dùng chính từ cần định nghĩa.

2. **T01-LT2:** Vì sao phải nói điện áp giữa hai điểm, nhưng nói dòng qua một nhánh?

3. **T01-LT3:** Dòng quy ước và chiều chuyển động electron trong kim loại có nhất thiết cùng chiều không?

4. **T01-LT4:** Pin đã có điện áp nhưng công tắc mở: đại lượng nào bằng 0 trong mô hình DC và đại lượng nào có thể khác 0?

5. **T01-LT5:** Nguồn 5 V/2 A có bắt mọi tải hút 2 A không? Giải thích vai trò của nguồn và tải.

6. **T01-LT6:** Vì sao dòng đi qua tải không bị tiêu hao, nhưng pin vẫn hết năng lượng?

### E. Bài tập tính toán hoặc thiết kế

**T01-B1.** Nguồn lý tưởng 3 V qua R = 1 kΩ trong 20 s. Tính I, Q, P, E và kiểm tra E = P × t = V × Q.

**T01-B2.** Giữ R = 1 kΩ: so nguồn 3 V và 6 V. Sau đó giữ nguồn 3 V: so R = 1 kΩ và 2 kΩ. Lập bảng I và P, giải thích từng tỷ lệ.

### F. Bài thực hành theo trình tự

**TH1.** Vẽ pin, công tắc và điện trở thành vòng. Đánh dấu hai điểm để nói về điện áp, một tiết diện để nói về dòng. Mở công tắc và viết đại lượng nào có thể còn khác 0.

**TH2.** Với nguồn lý tưởng 3 V và R = 1 kΩ, tính I = 3 mA. Trong 10 s, Q = 0,03 C; tải nhận E = 0,09 J; công suất P = 0,009 W. Viết ý nghĩa từng đáp số.

**TH3.** Tính lại khi R = 2 kΩ, rồi khi nguồn là 6 V và R = 1 kΩ. Dự đoán dòng và công suất, không chỉ thay vào công thức.

**TH4.** Đóng tài liệu và giải thích cho một người chưa học điện: vì sao không nói “dòng ở điểm này là 3 V”; vì sao nguồn 5 V/2 A không mặc định đẩy 2 A qua mọi tải.

**Bài biến thể và kiểm tra sâu:** Vẽ bốn sơ đồ: nguồn–R kín; công tắc mở; hai que đo điện áp; một vị trí quan sát dòng. Dùng mũi tên riêng cho dòng điện và lời ghi riêng cho năng lượng, tránh trộn hai khái niệm.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T01-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T01-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Trả lời được 6 câu bằng lời riêng; giải đúng cả hai bài có đơn vị; giải thích được vì sao một mạch hở vẫn có thể có điện áp.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-02"></a>
## Tuần 02 — Đo lường là một phần của mạch

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 1. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Chọn đúng cách mắc không cần nhắc; biết tác động của dụng cụ đo; giữ ảnh đặt que để người khác kiểm tra.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [Adafruit — Breadboards](https://learn.adafruit.com/breadboards-for-beginners/breadboards) | Đọc phần cấu trúc nhóm lỗ và power rails; đối chiếu breadboard thực. | Bản đồ nối điện của bảng cắm. |
| 2 | [Adafruit — Multimeters](https://learn.adafruit.com/multimeters) | Đọc lần lượt Continuity, Resistance và Voltage; Current đọc nguyên lý, chưa thực hành đo trực tiếp. | Bảng chế độ–cổng que–cách mắc–trạng thái nguồn. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** đồng hồ biết điện áp bằng cách nào, và việc đo có làm thay đổi mạch không?

**Bản chất.** Vôn kế mắc giữa hai điểm và có điện trở đầu vào hữu hạn, thường lớn, để hạn chế dòng nó lấy từ mạch. Vì vẫn là một tải, nó có thể làm sai lệch mạch có điện trở lớn. Ampe kế đặt nối tiếp và có trở kháng nhỏ; nối ngang nguồn có thể tạo đường dòng lớn. Ôm kế tự kích một tín hiệu đo, vì vậy không đo điện trở trên mạch đang có nguồn.

Breadboard chỉ nối các nhóm lỗ bằng thanh kim loại ẩn. Hai linh kiện trông gần nhau chưa chắc nối điện; hai điểm ở xa có thể cùng một nút. Một nút điện là tập hợp điểm được nối bằng dây có sụt áp bỏ qua trong mô hình đang dùng.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T02-LT1:** Vì sao vôn kế cần trở kháng đầu vào lớn?

2. **T02-LT2:** Vì sao ampe kế mắc ngang nguồn có thể tạo dòng lớn?

3. **T02-LT3:** Vì sao đo điện trở khi mạch còn điện có thể sai hoặc làm hỏng đồng hồ?

4. **T02-LT4:** Đảo que đo điện áp làm đổi dấu vì sao? Có phải nguồn đổi cực không?

5. **T02-LT5:** Thông mạch chứng minh được gì và chưa chứng minh được gì về dây cấp servo?

6. **T02-LT6:** Một số đo dao động: liệt kê ba nguyên nhân và cách tách ảnh hưởng của từng nguyên nhân.

### E. Bài tập tính toán hoặc thiết kế

**T02-B1.** Nguồn lý tưởng 3 V qua R = 1 MΩ. Đặt vôn kế có điện trở đầu vào 10 MΩ từ đầu cuối R xuống cực âm, hoàn thành vòng kín. Đồng hồ đọc bao nhiêu? So với giả định vôn kế không lấy dòng.

**T02-B2.** Điện trở nhãn 1 kΩ, dung sai ±5%. Tính khoảng cho phép. Số đo 980 Ω có phù hợp nhãn không, và chỉ phép so này đã chứng minh đồng hồ chính xác chưa?

### F. Bài thực hành theo trình tự

**TH1.** Chưa gắn nguồn. Chạm hai que đo để kiểm tra continuity, rồi lập bản đồ nhóm lỗ breadboard; kiểm tra thanh nguồn có bị ngắt ở giữa.

**TH2.** Đo từng pin và hộp hai pin. Đảo hai que đo ở chế độ điện áp và giải thích dấu âm. Đây là đổi mốc đo, không phải pin đổi cực.

**TH3.** Đo điện trở rời 330 Ω và 1 kΩ. Ghi giá trị thực, không làm tròn thành nhãn. Phân biệt sai số linh kiện, sai số đồng hồ và tiếp xúc.

**TH4.** Đo cùng một điện áp ba lần, ghi khoảng biến thiên. Tự chọn lại chế độ và cổng đo từ đầu. Chưa cần đo dòng trực tiếp; dùng điện áp trên điện trở để suy dòng ở bài sau.

**Bài biến thể và kiểm tra sâu:** Lập bảng 10 số đo: hai pin riêng, hộp pin, ba điện trở, bốn cặp lỗ breadboard. Với mỗi hàng ghi chế độ, cách đặt que và nguồn đã bật/tắt. Không đo điện trở trên mạch có điện.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T02-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T02-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Chọn đúng cách mắc không cần nhắc; biết tác động của dụng cụ đo; giữ ảnh đặt que để người khác kiểm tra.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-03"></a>
## Tuần 03 — LED không phải điện trở

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 2. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Suy được công thức; chọn điện trở có định mức phù hợp; giải thích được sai lệch bằng số đo thay vì nói chung là sai số.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [Ohm’s Law](https://www.allaboutcircuits.com/textbook/direct-current/chpt-2/voltage-current-resistance-relate/) | Ôn bài Ohm, sau đó trong sách DC đọc Power in Electric Circuits và Calculating Electric Power. | Tự suy ra P = I²R và P = V²/R từ P = VI. |
| 2 | [All About Circuits — Direct Current](https://www.allaboutcircuits.com/textbook/direct-current/) | Trong mục Semiconductors đọc Introduction to Diodes and Rectifiers và Diode Ratings; tập trung đặc tính phi tuyến và định mức. | Bảng phân biệt điện trở với LED. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** vì sao LED cần hạn dòng, và vì sao không thể coi LED luôn rơi đúng 2 V?

**Bản chất.** LED là diode phát quang với quan hệ dòng–áp phi tuyến. Trong vùng hoạt động, thay đổi nhỏ điện áp có thể làm dòng thay đổi nhiều. Điện trở nối tiếp giúp giới hạn dòng. Điện áp nguồn phân bố trên điện trở và LED: Vnguồn = VR + VLED trong mô hình bỏ qua dây. Vì nối tiếp nên dòng qua cả hai bằng nhau.

Công thức gần đúng I = (Vnguồn − VLED)/R cần một giá trị VLED ước lượng đúng loại LED và dòng đang xét. Để kiểm chứng, đo VR rồi dùng I = VR/R. Không giả sử mọi LED, màu sắc và nhiệt độ đều có VLED = 2 V.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T03-LT1:** Vì sao không thể lấy một giá trị điện trở cố định để mô tả LED ở mọi dòng?

2. **T03-LT2:** Công thức I = (Vs − Vf)/R dùng những giả thiết nào?

3. **T03-LT3:** Điện trở nối trước hay sau LED trong cùng mạch nối tiếp có khác nhau không?

4. **T03-LT4:** Vì sao cần đo Vf thay vì mặc định mọi LED rơi 2 V?

5. **T03-LT5:** Dòng trước LED và sau LED có bằng nhau trong trạng thái ổn định không? Giải thích bằng bảo toàn điện tích.

6. **T03-LT6:** Định mức tuyệt đối trong datasheet khác điều kiện hoạt động khuyến nghị thế nào?

### E. Bài tập tính toán hoặc thiết kế

**T03-B1.** Nguồn 3,3 V; giả sử LED có Vf = 2,0 V ở dòng mong muốn 4 mA. Tính R lý tưởng, chọn R = 330 Ω rồi tính I và công suất R.

**T03-B2.** Giữ nguồn 3,3 V và R = 330 Ω. Tính dòng nếu Vf lần lượt là 1,8 V và 2,2 V. Giải thích vì sao cùng R nhưng dòng khác nhau.

### F. Bài thực hành theo trình tự

**TH1.** Với giả thiết nguồn 3 V, LED đỏ 2 V, tính dòng khi R là 1 kΩ và 330 Ω: khoảng 1 mA và 3,03 mA. Tính công suất trên R bằng I²R.

**TH2.** Lắp pin dương → R 1 kΩ → anode LED → cathode LED → pin âm. Xác định cực LED từ datasheet/marking, không chỉ suy từ chiều dài chân đã bị cắt.

**TH3.** Đo nguồn dưới tải, VR và VLED. Kiểm tra tổng hai sụt áp gần bằng nguồn. Tính dòng từ R đo thực.

**TH4.** Ngắt nguồn, đổi sang 330 Ω. Lặp phép đo và giải thích sai lệch so với dự đoán. Giữ ảnh/sơ đồ của cả hai cấu hình.

**Bài biến thể và kiểm tra sâu:** Đo hai cấu hình R = 1 kΩ và 330 Ω. Mỗi cấu hình ghi Vs, VR, VLED, R đo thực và I suy ra; kiểm tra cân bằng điện áp. Ghi cả dự đoán trước khi đo.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T03-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T03-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Suy được công thức; chọn điện trở có định mức phù hợp; giải thích được sai lệch bằng số đo thay vì nói chung là sai số.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-04"></a>
## Tuần 04 — Nút điện và bảo toàn dòng

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 3. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Nhận diện series/parallel từ nút nối; tự lập KCL/KVL; tìm lỗi hở có bằng chứng.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [Series and Parallel Circuits](https://www.allaboutcircuits.com/textbook/direct-current/chpt-5/what-are-series-and-parallel-circuits/) | Đọc định nghĩa và sơ đồ series/parallel; vẽ lại bằng cách bố trí khác. | Xác định cùng nút thay vì dựa vị trí hình vẽ. |
| 2 | [All About Circuits — Direct Current](https://www.allaboutcircuits.com/textbook/direct-current/) | Đọc chương Kirchhoff’s Laws: Kirchhoff’s Current Law và Kirchhoff’s Voltage Law. | Hai phương trình bảo toàn cho mạch tự vẽ. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** làm sao nhìn sơ đồ để biết dòng đi qua những đường nào?

**Bản chất.** Các phần tử nối tiếp thật sự có chung một dòng khi nút giữa chúng không có nhánh khác. Các nhánh song song nối cùng hai nút nên có cùng điện áp. Dòng tại nút tuân theo bảo toàn điện tích; tổng dòng vào bằng tổng dòng ra ở trạng thái ổn định. Vòng kín tuân theo cân bằng điện áp trong mô hình mạch tập trung DC.

Công tắc lý tưởng đóng gần như dây nối; mở gần như đường hở. Công tắc không tạo điện áp. Nút bốn chân thường có hai cặp nối sẵn; nhấn nối hai nhóm, nhưng phải đo nút thực tế để biết.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T04-LT1:** Hai điểm nào được coi là cùng một nút trong mô hình dây lý tưởng?

2. **T04-LT2:** Điều kiện để hai linh kiện thật sự nối tiếp là gì?

3. **T04-LT3:** Vì sao mắc song song cho cùng điện áp nhưng không nhất thiết cùng dòng?

4. **T04-LT4:** KCL gắn với bảo toàn đại lượng nào? KVL trong mạch DC đơn giản gắn với điều gì?

5. **T04-LT5:** Tại sao nút bốn chân có thể bị nối sai khiến nhấn/thả không thay đổi mạch?

6. **T04-LT6:** Một mạch thay đổi hình vẽ nhưng giữ nguyên nút nối có đổi hành vi không?

### E. Bài tập tính toán hoặc thiết kế

**T04-B1.** Nguồn 6 V qua R1 = 1 kΩ nối tiếp R2 = 2 kΩ. Tính dòng, sụt áp và công suất từng R.

**T04-B2.** Hai R trên mắc song song qua 6 V. Tính dòng từng nhánh, dòng tổng, R tương đương và tổng công suất.

### F. Bài thực hành theo trình tự

**TH1.** Dùng continuity xác định các chân nút khi nhấn/thả, không cấp nguồn.

**TH2.** Thêm nút nối tiếp mạch LED. Vẽ đường đi khi đóng và vị trí hở khi thả.

**TH3.** Trên giấy, so sánh hai R 1 kΩ nối tiếp và song song qua 3 V: tương đương 2 kΩ và 500 Ω; dòng tổng 1,5 mA và 6 mA. Lắp bằng điện trở, kiểm tra sụt áp; không thay bằng hai LED và áp nguyên công thức điện trở.

**TH4.** Ngắt nguồn, tháo một dây rồi tìm điểm hở bằng đo và sơ đồ. Lắp lại hoàn toàn từ sơ đồ nguyên lý.

**Bài biến thể và kiểm tra sâu:** Tự lắp hai mạch điện trở bằng nguồn pin, dùng số nguồn thực vào phép tính. Dùng nút điều khiển LED rồi tạo một lỗi hở dây khi đã ngắt nguồn; khoanh vùng lỗi với tối đa ba phép đo có lý do.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T04-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T04-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Nhận diện series/parallel từ nút nối; tự lập KCL/KVL; tìm lỗi hở có bằng chứng.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-05"></a>
## Tuần 05 — Chia áp tải và đầu vào thả nổi

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 4. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Tự tính lại khi có tải; không đồng nhất đầu vào thả nổi với mức 0; biết tại sao phải có giới hạn dòng.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [Voltage Divider Circuits](https://www.allaboutcircuits.com/textbook/direct-current/chpt-6/voltage-divider-circuits/) | Đọc đầy đủ phần suy công thức và ảnh hưởng tải nếu có; tự thay tải bằng điện trở tương đương. | Ba sơ đồ không tải/có tải/biến trở. |
| 2 | [ESP32-S3 GPIO](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/peripherals/gpio.html) | Chỉ đọc IO Configuration liên quan input và internal pull-up/down; đối chiếu giới hạn điện khi thực hành về sau. | Giải thích floating input bằng mô hình điện. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** vì sao mạch chia đôi điện áp lại không còn chia đôi sau khi gắn thêm thiết bị?

**Bản chất.** Với R1 ở trên, R2 ở dưới, đầu ra không tải là Vout = Vin × R2/(R1+R2). Công thức suy từ dòng chung I = Vin/(R1+R2), sau đó Vout = I × R2. Khi mắc tải RL từ đầu ra xuống GND, nhánh dưới trở thành R2 song song RL. Không còn được thay nguyên R2 cũ vào công thức.

Ví dụ nguồn 3 V, R1 = R2 = 1 kΩ: Vout = 1,5 V. Thêm RL = 1 kΩ: nhánh dưới còn 500 Ω, Vout = 1 V. Đây là lý do mạch chia áp thông thường không thay được một nguồn ổn áp cấp motor.

Đầu vào số có trở kháng cao nên lấy ít dòng. Nếu không có đường xác định điện áp, nhiễu và điện tích có thể chi phối mức đọc. Pull-up/pull-down là điện trở tạo đường về mức mặc định; không phải tăng sức mạnh tín hiệu một cách bí ẩn.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T05-LT1:** Suy Vout từ dòng chung thay vì nhớ công thức.

2. **T05-LT2:** Khi gắn tải xuống GND, điện trở nào bị thay bằng tổ hợp song song?

3. **T05-LT3:** Vì sao bộ chia áp không phải nguồn ổn áp cấp motor?

4. **T05-LT4:** Biến trở ba chân khác dùng hai chân như điện trở biến đổi thế nào?

5. **T05-LT5:** Pull-up tạo mức mặc định bằng đường dòng nào?

6. **T05-LT6:** Tăng pull-up làm đổi dòng khi nhấn và độ nhạy nhiễu ra sao ở mức định tính?

### E. Bài tập tính toán hoặc thiết kế

**T05-B1.** Vin = 3 V; R1 = R2 = 1 kΩ. Tính Vout không tải, rồi thêm RL = 1 kΩ từ Vout xuống GND.

**T05-B2.** Nút nối input xuống GND; pull-up 10 kΩ lên 3,3 V. Tính dòng pull-up khi nhấn và công suất R. Nếu thay bằng 1 kΩ thì sao?

### F. Bài thực hành theo trình tự

**TH1.** Tự suy ra công thức và làm ví dụ 1,5 V → 1 V.

**TH2.** Lắp ba điện trở như ví dụ, đo trước/sau gắn tải; dùng điện áp nguồn đo thực.

**TH3.** Dùng biến trở 10 kΩ làm chia áp; đo đầu ra ở ba vị trí.

**TH4.** Vẽ nút kéo xuống GND với điện trở kéo lên. Tính dòng qua R kéo khi nhấn; giải thích vì sao không nối nút trực tiếp giữa hai rail nguồn.

**Bài biến thể và kiểm tra sâu:** Đo bài chia áp trước/sau gắn tải; vẽ đường dòng pull-up ở hai trạng thái; kiểm tra biến trở bằng đồng hồ trước khi dùng nó làm đầu vào MCU.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T05-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T05-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Tự tính lại khi có tải; không đồng nhất đầu vào thả nổi với mức 0; biết tại sao phải có giới hạn dòng.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-06"></a>
## Tuần 06 — Tụ điện thời gian và mô hình gần đúng

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 5. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Giải thích được hình dạng đồ thị, không chỉ thuộc 63%; phân biệt thời gian đáp ứng với điện áp cuối.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [All About Circuits — Direct Current](https://www.allaboutcircuits.com/textbook/direct-current/) | Đọc Electric Fields and Capacitance, Practical Considerations — Capacitors và Capacitor Transient Response. Trong mục bán dẫn đọc The BJT as a Switch ở mức nguyên lý. | Một đồ thị nạp/xả, một bảng phân biệt R/C/diode/transistor. |
| 2 | [OpenStax — University Physics Volume 2](https://openstax.org/books/university-physics-volume-2/pages/9-1-electrical-current) | Dùng mục lục sách đến chương Capacitance khi muốn suy sâu Q = CV và năng lượng tụ; không bắt buộc giải tích. | Giải thích tụ chứa năng lượng trong điện trường. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** vì sao điện áp tụ thay đổi dần, và vì sao tụ lớn không chữa được mọi vấn đề nguồn?

**Bản chất.** Tụ lưu năng lượng trong điện trường. Với tụ lý tưởng, Q = C × V và I = C × dV/dt. Cùng một dòng, tụ lớn hơn thay đổi điện áp chậm hơn. Trong mạch nguồn áp–điện trở–tụ, lúc tụ chưa nạp, dòng lớn nhất theo mô hình; điện áp tụ tăng làm điện áp trên R giảm, nên dòng nạp giảm dần.

Hằng số thời gian τ = RC. Với tụ bắt đầu 0 V nạp từ nguồn không đổi: sau 1τ khoảng 63%, sau 3τ khoảng 95%, sau 5τ khoảng 99,3% điện áp cuối. “Sau RC thì đầy” là sai. Không cần học giải phương trình vi phân ngay; phải hiểu tại sao tốc độ nạp giảm.

Tụ nguồn chỉ hỗ trợ trong một khoảng thời gian. Xấp xỉ ΔV = I × Δt/C cho đoạn dòng gần không đổi. Thiếu 0,1 A trong 1 ms với tụ 100 µF đã tương ứng 1 V sụt theo mô hình lý tưởng; tụ không thay nguồn thiếu khả năng cấp dòng lâu dài. Tụ thực còn có điện trở/điện cảm ký sinh.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T06-LT1:** Vì sao điện áp tụ không thể nhảy tức thì nếu dòng hữu hạn trong mô hình tụ lý tưởng?

2. **T06-LT2:** Vì sao dòng nạp RC giảm dần?

3. **T06-LT3:** Sau một RC tụ đã đầy chưa?

4. **T06-LT4:** Tụ thực khác lý tưởng ở những yếu tố nào?

5. **T06-LT5:** Tụ bổ sung có chữa được nguồn thiếu công suất liên tục không?

6. **T06-LT6:** Diode, transistor và tụ có vai trò khác nhau thế nào? Chỉ ra chân từ datasheet một linh kiện thực.

### E. Bài tập tính toán hoặc thiết kế

**T06-B1.** R = 100 kΩ; C = 100 µF; nguồn 3 V, tụ ban đầu 0 V. Tính τ và điện áp gần đúng sau 10, 30, 50 s.

**T06-B2.** Tụ lý tưởng 100 µF phải cấp riêng 0,1 A trong 1 ms. Tính mức sụt áp. Muốn chỉ sụt 0,1 V trong điều kiện đó cần C bao nhiêu?

### F. Bài thực hành theo trình tự

**TH1.** Tính τ với 100 kΩ và 100 µF là 10 s; dự đoán điện áp ở 10, 30, 50 s với nguồn 3 V.

**TH2.** Lắp đúng cực, đo điện áp tụ theo thời gian. Đồng hồ và dòng rò có thể gây sai lệch; ghi điều kiện đo.

**TH3.** Ngắt nguồn và xả qua điện trở, quan sát điện áp giảm; không chập trực tiếp tụ để xả.

**TH4.** Đọc datasheet diode/BJT/MOSFET ở mức vai trò và pinout. Phân biệt diode dẫn có chiều, transistor làm khóa điều khiển và tụ lưu năng lượng. Chưa cần thiết kế khuếch đại.

**Bài biến thể và kiểm tra sâu:** Đo Vc theo thời gian, tối thiểu sáu điểm; đổi R một lần để kiểm tra tỷ lệ thời gian. Ngắt nguồn, xả qua R trước thao tác. Nếu số đo lệch lớn, xét tải đồng hồ và dòng rò.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T06-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T06-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Giải thích được hình dạng đồ thị, không chỉ thuộc 63%; phân biệt thời gian đáp ứng với điện áp cuối.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-07"></a>
## Tuần 07 — Chương trình trở thành hành vi phần cứng như thế nào

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 6. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Tự phân loại lỗi build/nạp/boot/chạy; xác nhận đúng bo; tái nạp được từ ghi chú.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [MakerEdu — MKE-K01](https://github.com/makereduvn/MKE-K01-ESP32-S3-DEV-KIT) | Đọc giới thiệu, pinout, schematic và hướng dẫn BOOT/RESET đúng phiên bản bo. | Bảng nhận dạng bo và các chân dành riêng. |
| 2 | [ESP32-S3 NVS](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/storage/nvs_flash.html) | Đọc Introduction và phần kiểu dữ liệu/cách commit; chưa đọc toàn bộ API. | Phân biệt RAM, flash firmware và nơi lưu cấu hình. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** từ file mã nguồn đến một dòng log trên laptop đã xảy ra những gì?

**Bản chất.** CPU thực thi lệnh máy, không đọc trực tiếp mã Rust. Build tạo mã và liên kết thành firmware cho đúng kiến trúc; nạp ghi dữ liệu vào flash; chuỗi khởi động chuẩn bị hệ thống rồi chạy ứng dụng. Reset khởi động lại thực thi, không mặc định xóa flash.

Flash giữ chương trình sau mất nguồn; RAM chứa dữ liệu làm việc. Stack phục vụ các khung gọi và biến theo cơ chế của chương trình; heap phục vụ cấp phát động. PSRAM bổ sung dung lượng nhưng có đặc tính truy cập khác RAM trong chip. Không lấy tổng dung lượng RAM để suy rằng mọi ngoại vi dùng được mọi buffer.

HAL là lớp phần mềm giúp cấu hình ngoại vi; nó không bỏ qua giới hạn điện hay tự biết bạn cắm dây vào đâu.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T07-LT1:** Vẽ các bước source → compile/link → firmware → flash → boot → application.

2. **T07-LT2:** Vì sao chương trình desktop không chạy nguyên trên ESP32?

3. **T07-LT3:** Reset, mất nguồn và xóa flash khác nhau thế nào?

4. **T07-LT4:** Flash, RAM trong chip và PSRAM phục vụ những việc nào?

5. **T07-LT5:** HAL có thể bảo vệ bạn khỏi cắm sai điện áp không?

6. **T07-LT6:** Tại sao cần lưu phiên bản toolchain, dependency và cấu hình build?

### E. Bài tập tính toán hoặc thiết kế

**T07-B1.** Ảnh firmware giả định 2,5 MiB; chip có 16 MiB flash. Có thể khẳng định còn 13,5 MiB cho ứng dụng tự dùng không? Nêu tối thiểu ba yếu tố phải kiểm tra.

**T07-B2.** Một buffer chứa 16.000 mẫu, mỗi mẫu 4 byte. Tính byte/KiB. Có thể khẳng định buffer DMA đặt được ở PSRAM chỉ vì còn đủ dung lượng không?

### F. Bài thực hành theo trình tự

**TH1.** Chụp bo, ghi mã biến thể và đánh dấu nguồn, GND, USB, BOOT, RESET. Lưu schematic đúng bo.

**TH2.** Nạp chương trình in số đếm mỗi giây. Ghi phiên bản toolchain, ESP-IDF và dependency.

**TH3.** Reset, rút/cắm lại; quan sát số đếm. Đổi thông điệp rồi nạp lại để xác nhận đang chạy đúng bản.

**TH4.** Viết sơ đồ source → build → firmware → flash → boot → log. Phân loại một lỗi build, một lỗi kết nối nạp và một lỗi chương trình.

**Bài biến thể và kiểm tra sâu:** Nạp log có mã phiên bản; reset và cấp nguồn lại; ghi cách phục hồi khi không nạp được. Lưu một cấu hình nhỏ nếu môi trường đã hỗ trợ NVS và chứng minh nó còn sau restart.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T07-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T07-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Tự phân loại lỗi build/nạp/boot/chạy; xác nhận đúng bo; tái nạp được từ ghi chú.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-08"></a>
## Tuần 08 — GPIO là mạch điện điều khiển được

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 7. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Nối được LED có tính dòng; đọc được giới hạn thay vì áp số trên mạng; phân biệt tín hiệu và cấp công suất.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [ESP32-S3 GPIO](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/peripherals/gpio.html) | Đọc GPIO Summary, IO Configuration và chú thích chân bị ràng buộc; từ trang mở datasheet phần đặc tính điện. | Bảng chân sử dụng/không sử dụng có lý do. |
| 2 | [MakerEdu — MKE-K01](https://github.com/makereduvn/MKE-K01-ESP32-S3-DEV-KIT) | Đối chiếu pinout và schematic của đúng bo; kiểm tra LED tích hợp là LED thường hay LED địa chỉ. | Sơ đồ mạch LED rời. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** lệnh đặt chân HIGH đã thay đổi điều gì bên trong chip?

**Bản chất.** Chân GPIO đầu ra điều khiển một tầng transistor nối ra mức điện gần nguồn hoặc gần GND theo cấu hình. Nó không phải nguồn điện lý tưởng; dòng cho phép và sụt áp có giới hạn. Chân đầu vào đo/diễn giải điện áp theo các ngưỡng logic, không có nghĩa mọi điện áp lớn hơn 0 đều là số 1.

Mức HIGH và LOW là quy ước theo điện áp tham chiếu, vùng chưa xác định và điều kiện datasheet. Thiết bị cần mốc tham chiếu phù hợp để hiểu tín hiệu. Điều khiển servo bằng GPIO là truyền lệnh, không phải cấp toàn bộ năng lượng motor qua chân đó.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T08-LT1:** Một lệnh HIGH tác động lên tầng output thế nào ở mức khối?

2. **T08-LT2:** Vì sao GPIO không phải nguồn áp lý tưởng?

3. **T08-LT3:** Input diễn giải HIGH/LOW dựa trên ngưỡng nào?

4. **T08-LT4:** Vì sao hai mạch truyền tín hiệu cần mốc tham chiếu phù hợp?

5. **T08-LT5:** Output push-pull khác open-drain ở mức ý tưởng thế nào?

6. **T08-LT6:** Tại sao không suy mọi chân header đều dùng được tùy ý?

### E. Bài tập tính toán hoặc thiết kế

**T08-B1.** Ước lượng dòng LED với GPIO 3,3 V, Vf = 2 V, R = 1 kΩ. Nếu đo HIGH dưới tải là 3,2 V thì dòng ước lượng đổi ra sao?

**T08-B2.** Giả định input yêu cầu LOW ≤0,8 V và HIGH ≥2,0 V. Phân loại 0,4 V, 1,2 V, 2,8 V. Đây có phải thông số mặc định ESP32-S3 không?

### F. Bài thực hành theo trình tự

**TH1.** Chọn chân output hợp lệ. Tính gần đúng dòng LED với nguồn logic 3,3 V, LED đỏ ước lượng 2 V, R 1 kΩ: khoảng 1,3 mA.

**TH2.** Nối LED rời với R, bật/tắt mỗi giây. Nếu LED tích hợp dùng giao thức riêng, không dùng nó để thay bài GPIO đơn giản.

**TH3.** Giữ LOW rồi HIGH để đo điện áp chân so với GND; không dùng số đo trung bình lúc chớp để kết luận mức logic.

**TH4.** Thay nhịp sáng/tắt và vẽ tín hiệu theo thời gian. Đối chiếu tải và thông số chân.

**Bài biến thể và kiểm tra sâu:** Giữ LOW/HIGH và đo từng mức; sau đó tạo nhịp chớp. Tự đổi chân chỉ sau đối chiếu bảng chân; ghi những cấu hình phải đổi và những phần không đổi.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T08-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T08-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Nối được LED có tính dòng; đọc được giới hạn thay vì áp số trên mạng; phân biệt tín hiệu và cấp công suất.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-09"></a>
## Tuần 09 — Từ tiếp điểm cơ khí đến một sự kiện nhấn

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 8. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Một lần nhấn hợp lệ sinh một sự kiện; giải thích chính xác điều kiện thời gian, không chỉ nói đã thêm delay.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [Arduino — Debounce](https://docs.arduino.cc/built-in-examples/digital/Debounce/) | Đọc mô tả dội nút và thuật toán thời gian; vẽ lại bằng lưu đồ tiếng Việt. | Luật chấp nhận nhấn/thả. |
| 2 | [ESP32-S3 GPIO](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/peripherals/gpio.html) | Đọc input, pull-up/down và interrupt ở mức nguyên lý; chưa cần tối ưu interrupt. | So sánh polling và interrupt. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** một lần tay nhấn có nhất thiết tương ứng một lần mức điện đổi không?

**Bản chất.** Nút có hai tầng ý nghĩa: tín hiệu điện tức thời và sự kiện người dùng mong muốn. Tiếp điểm có thể đóng/mở nhiều lần trong một khoảng ngắn. Polling đọc theo chu kỳ có thể bắt nhiều cạnh hoặc bỏ lỡ xung ngắn. Debounce định nghĩa điều kiện để chấp nhận một trạng thái ổn định; nó đánh đổi độ trễ với khả năng bỏ nhiễu.

Pull-up xác lập mức khi nút hở. Khi nút nối chân xuống GND, chân xuống LOW; dòng chạy từ nguồn qua điện trở kéo và nút về GND. Đếm mỗi lần đọc LOW sẽ tăng liên tục khi giữ; đếm cạnh chuyển trạng thái đã chống dội mới gần với “một lần nhấn”.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T09-LT1:** Trạng thái điện và sự kiện nhấn khác nhau thế nào?

2. **T09-LT2:** Đếm LOW mỗi vòng lặp khác đếm cạnh xuống ra sao?

3. **T09-LT3:** Vì sao interrupt không tự giải quyết dội?

4. **T09-LT4:** Debounce quá ngắn/quá dài có nhược điểm gì?

5. **T09-LT5:** Nếu polling quá chậm thì loại thao tác nào dễ bỏ lỡ?

6. **T09-LT6:** Vì sao một lần đầu vào floating đọc ổn chưa chứng minh mạch đúng?

### E. Bài tập tính toán hoặc thiết kế

**T09-B1.** Input bắt đầu HIGH. Mẫu mức tại t(ms): 0:H, 3:L, 7:H, 11:L, 15:H, 19:L và giữ LOW đến 100. Với quy tắc cần ổn định LOW đủ 20 ms, sớm nhất chấp nhận nhấn lúc nào, nếu quan sát liên tục?

**T09-B2.** Polling mỗi 50 ms tại 0,50,100... Một xung nhấn từ 10 đến 30 ms có được thấy không? Nêu cách thiết kế để đáp ứng loại xung này.

### F. Bài thực hành theo trình tự

**TH1.** Lắp nút với pull-up phù hợp, log mức và thời điểm. Vẽ đường dòng khi thả/nhấn.

**TH2.** So sánh đếm theo mức và theo cạnh. Giữ nút hai giây để thấy khác biệt.

**TH3.** Thêm điều kiện ổn định bắt đầu 20–50 ms, đo thực tế để điều chỉnh; không coi một giá trị là đúng cho mọi nút.

**TH4.** Nhấn 20 lần riêng biệt, giữ lâu, nhấn nhanh và thả chậm. Ghi ca bị bỏ/lặp và lý do.

**Bài biến thể và kiểm tra sâu:** Log mức raw và sự kiện đã lọc trong cùng trục thời gian; thử 20 nhấn, giữ 2 s và hai nhấn gần nhau. Báo cáo những loại thao tác được hỗ trợ.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T09-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T09-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Một lần nhấn hợp lệ sinh một sự kiện; giải thích chính xác điều kiện thời gian, không chỉ nói đã thêm delay.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-10"></a>
## Tuần 10 — ADC lấy mẫu và PWM điều chế thời gian

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 9. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Đổi đúng đơn vị và dải đo; phân biệt dữ liệu raw, điện áp suy ra và PWM.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [ESP32-S3 LEDC PWM](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/peripherals/ledc.html) | Đọc Timer Configuration, Channel Configuration và quan hệ frequency–duty resolution. | Bảng chu kỳ–tần số–duty–độ rộng xung. |
| 2 | [ESP32-S3 ADC Oneshot](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/peripherals/adc/adc_oneshot.html) | Đọc Unit Configuration, Read Conversion Result, Hardware Limitations và liên kết Calibration; lấy phiên bản khớp dự án. | Bảng chân ADC, attenuation, giới hạn và cách chuyển raw thành điện áp. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** vì sao số đọc không phải điện áp chính xác, và vì sao xung bật/tắt lại điều khiển được độ sáng?

**Bản chất.** ADC lấy mẫu điện áp rồi lượng tử hóa về số mức hữu hạn. Độ phân giải N bit cho tối đa 2^N mã; độ chính xác còn phụ thuộc tham chiếu, nhiễu, tuyến tính và hiệu chuẩn. Công thức lý tưởng từ mã sang V chỉ đúng với mô hình dải đo đã chọn, không tự áp nguyên cho ESP32 ở mọi attenuation.

PWM giữ các mức xung nhưng thay tỷ lệ thời gian bật. Duty = thời gian bật/chu kỳ; f = 1/chu kỳ. Tải và hệ quan sát đáp ứng với chuỗi xung; chân không thực sự trở thành nguồn DC liên tục có điện áp bằng duty nhân nguồn. 50% duty không đảm bảo cảm nhận độ sáng bằng đúng nửa.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T10-LT1:** Độ phân giải ADC khác độ chính xác thế nào?

2. **T10-LT2:** ADC raw có tự bằng điện áp không?

3. **T10-LT3:** Tại sao lấy trung bình giảm một số dao động nhưng không sửa mọi sai lệch?

4. **T10-LT4:** PWM thay mức điện áp hay thay thời gian ở các mức?

5. **T10-LT5:** Tại sao cùng duty nhưng khác tần số có thể có tác động khác?

6. **T10-LT6:** Tại sao tăng độ phân giải duty và tăng tần số không thể tùy ý đồng thời?

### E. Bài tập tính toán hoặc thiết kế

**T10-B1.** ADC lý tưởng 12 bit, quy ước dải 0–3,3 V tương ứng mã 0–4095. Tính bước theo quy ước này và điện áp mã 2048. Nêu lý do không áp thẳng cho mọi cấu hình ESP32.

**T10-B2.** PWM 1 kHz có duty 25%, 50%, 75%. Tính chu kỳ và thời gian bật. Giữ duty 25% nhưng đổi 2 kHz thì thời gian bật bao nhiêu?

### F. Bài thực hành theo trình tự

**TH1.** Chọn dải và chân ADC hợp lệ; thiết kế biến trở không vượt giới hạn đầu vào. Đo đồng thời điện áp bằng đồng hồ và mã ADC tại năm điểm.

**TH2.** Giữ nguyên vị trí, ghi 20 mẫu; tính min, max và trung bình. Giải thích dao động, không sửa bằng ép mọi giá trị về một số.

**TH3.** PWM LED ở 0, 25, 50, 75, 100%; tính thời gian bật ở một tần số được chọn và hỗ trợ.

**TH4.** Ghép biến trở điều khiển PWM, thử lấy trung bình một nhóm mẫu; quan sát giảm rung và độ trễ.

**Bài biến thể và kiểm tra sâu:** Đo biến trở ở 5 vị trí, mỗi vị trí 20 mẫu ADC; ghi min/max/trung bình và điện áp đồng hồ. Điều khiển độ sáng bằng ADC; thử lọc để thấy đánh đổi độ trễ.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T10-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T10-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Đổi đúng đơn vị và dải đo; phân biệt dữ liệu raw, điện áp suy ra và PWM.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-11"></a>
## Tuần 11 — I2C và ý nghĩa của một lần truyền thành công

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 10. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Tự tìm lỗi theo nguồn/dây/bus/driver; không chọn driver chỉ vì cùng kích thước màn hình.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [ESP32-S3 I²C](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/peripherals/i2c.html) | Đọc Introduction, cấu hình bus và device, địa chỉ và tốc độ. Tìm các khái niệm SDA/SCL, pull-up, ACK. | Chú thích một giao dịch I²C trên giấy. |
| 2 | [Adafruit — Monochrome OLED](https://learn.adafruit.com/monochrome-oled-breakouts) | Đọc pinout, giao tiếp và loại controller; áp đúng module thực dùng. | Bảng nguồn/mức logic/chip/địa chỉ/driver. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** CPU làm sao gọi đúng màn hình khi nhiều thiết bị dùng chung dây?

**Bản chất.** I²C dùng SDA để truyền bit và SCL làm nhịp. Đường open-drain được kéo xuống bởi thiết bị và được điện trở kéo lên khi thả. Điện trở và điện dung đường dây ảnh hưởng tốc độ lên mức cao; tăng tốc không chỉ là đổi một con số phần mềm.

Địa chỉ giúp chọn thiết bị; ACK là xác nhận ở mức giao dịch, không chứng minh màn hình đã vẽ đúng mắt. Địa chỉ 7 bit khác byte gồm địa chỉ cộng bit đọc/ghi. Giao thức bus, chip điều khiển màn hình và nội dung hình là ba lớp khác nhau.

UART dùng khung và tốc độ thống nhất mà không có dây clock riêng theo kiểu I²C; SPI thường có clock, các đường dữ liệu và chọn chip; I²S dành cho luồng âm thanh. Chưa cần mua module cho mọi bus.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T11-LT1:** Tại sao I²C dùng pull-up với đường open-drain?

2. **T11-LT2:** Ai kéo tín hiệu xuống và điều gì làm tín hiệu lên cao?

3. **T11-LT3:** ACK xác nhận điều gì, chưa xác nhận điều gì?

4. **T11-LT4:** Địa chỉ 7 bit khác byte địa chỉ có bit R/W thế nào?

5. **T11-LT5:** Tại sao tăng clock có thể khiến mạch từng chạy bị lỗi?

6. **T11-LT6:** Phân biệt I²C, UART, SPI, I²S về mục đích và đường tín hiệu.

### E. Bài tập tính toán hoặc thiết kế

**T11-B1.** Thiết bị địa chỉ 7 bit 0x3C. Nếu mô tả byte địa chỉ trên dây bằng ghép R/W, byte ghi và đọc là gì? API nhận địa chỉ 7 bit nên truyền gì?

**T11-B2.** Giả sử một giao dịch có 1 byte địa chỉ và 16 byte dữ liệu, mỗi byte có 8 bit cộng 1 ACK, clock 100 kHz. Ước lượng thời gian chỉ phần clock, bỏ START/STOP và chờ.

### F. Bài thực hành theo trình tự

**TH1.** Xác nhận chip và pinout OLED; vẽ nguồn, GND, SDA, SCL và mức kéo lên. Không suy chip từ kích thước 0,96 inch.

**TH2.** Quét địa chỉ ở tốc độ được hỗ trợ; ghi phản hồi.

**TH3.** Hiển thị chữ với driver đúng chip; ghi các bước từ lời gọi vẽ tới giao dịch bus.

**TH4.** Ngắt nguồn trước khi thay dây, tạo một ca thiếu SDA hoặc sai địa chỉ trong cấu hình rồi quan sát lỗi và phục hồi. Nếu có logic analyzer, đối chiếu START, address, ACK.

**Bài biến thể và kiểm tra sâu:** Quét địa chỉ; hiển thị chữ và bộ đếm. Tạo lỗi sai địa chỉ bằng cấu hình và ghi mã lỗi. Nếu có logic analyzer, chụp một giao dịch và gắn nhãn.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T11-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T11-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Tự tìm lỗi theo nguồn/dây/bus/driver; không chọn driver chỉ vì cùng kích thước màn hình.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-12"></a>
## Tuần 12 — Hình ảnh bộ nhớ và thời gian

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 11. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Tính được bộ nhớ/băng thông; giải thích vì sao ảnh chưa đổi hoặc nút bị trễ.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [Adafruit — Monochrome OLED](https://learn.adafruit.com/monochrome-oled-breakouts) | Đọc phần đồ họa, tọa độ và buffer; tập trung draw rồi update màn hình. | Sơ đồ CPU → buffer → bus → controller → pixel. |
| 2 | [Adafruit — Multi-tasking Part 1](https://learn.adafruit.com/multi-tasking-the-arduino-part-1/a-classy-solution) | Từ mục lục đọc Ditch the delay(), Using millis() for timing và Now for two at once. | Lịch sự kiện mắt và nút không chờ dài. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** một đôi mắt trên màn hình được biểu diễn bằng những dữ liệu nào?

**Bản chất.** Ảnh đơn sắc gán một bit cho mỗi pixel trong mô hình framebuffer. 128 × 64 pixel cần 8192 bit = 1024 byte dữ liệu hình; cấu trúc driver có thể cần thêm bộ nhớ. Tọa độ, buffer trong RAM, dữ liệu bus và trạng thái hiển thị là các lớp khác nhau.

Hoạt ảnh là chuỗi ảnh theo thời gian, không phải một thuộc tính đặc biệt của pixel. Nếu mọi lần cập nhật đều chờ dài, việc đọc nút cũng bị trì hoãn trong cùng luồng. Có thể lưu hạn thời gian kế tiếp thay vì ngồi chờ đến hạn.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T12-LT1:** Pixel và byte trong framebuffer liên hệ thế nào?

2. **T12-LT2:** Vẽ vào RAM đã có nghĩa màn hình thay đổi chưa?

3. **T12-LT3:** Hoạt ảnh được tạo bằng những trạng thái và thời điểm nào?

4. **T12-LT4:** Vì sao một delay dài trong luồng đọc nút có thể bỏ thao tác?

5. **T12-LT5:** Tăng FPS tốn tài nguyên nào?

6. **T12-LT6:** Khi nào lưu toàn bộ khung, khi nào vẽ theo tham số thuận tiện hơn?

### E. Bài tập tính toán hoặc thiết kế

**T12-B1.** Màn hình 128×64, 1 bit/pixel. Tính dung lượng 1 khung và 3 khung, theo byte và KiB.

**T12-B2.** Gửi đủ 1024 byte qua I²C 100 kHz, giả sử mỗi byte có 9 clock. Tính thời gian tối thiểu phần dữ liệu và tần số cập nhật lý tưởng tối đa, bỏ mọi overhead.

### F. Bài thực hành theo trình tự

**TH1.** Vẽ một điểm, một hình chữ nhật và hai mắt. Thay tọa độ x/y độc lập để xác nhận gốc và chiều trục.

**TH2.** Tạo ba khung mở/nhắm/vui. Tính bộ nhớ nếu lưu ba ảnh đầy đủ và so với vẽ hình theo tham số.

**TH3.** Mở 3000 ms, nhắm 150 ms bằng trạng thái và mốc thời gian; nút vẫn được đọc.

**TH4.** Tăng nhịp vẽ, quan sát phản hồi nút và tải truyền. Chỉ gửi khi cần hoặc theo nhịp hợp lý.

**Bài biến thể và kiểm tra sâu:** Vẽ ba biểu cảm. Cho mắt chớp độc lập với nút; đổi nhịp 1 s và 3 s mà nút vẫn nhận. Đo số update thay vì chỉ quan sát video.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T12-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T12-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Tính được bộ nhớ/băng thông; giải thích vì sao ảnh chưa đổi hoặc nút bị trễ.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-13"></a>
## Tuần 13 — Servo là một hệ điều khiển có phản hồi bên trong

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 12. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Phân biệt điều khiển ngoài với phản hồi trong servo; không áp nhầm PWM LED; chuyển động không chạm chặn.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [Adafruit — Servo Motors](https://learn.adafruit.com/adafruit-arduino-lesson-14-servo-motors) | Đọc Servo Motors, Hardware và phần điều khiển; tập trung cấu tạo và vị trí, không học cú pháp Arduino. | Sơ đồ motor–hộp số–cảm biến–bộ điều khiển. |
| 2 | [ESP32-S3 LEDC PWM](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/peripherals/ledc.html) | Ôn chu kỳ và độ rộng xung; tra timer để tạo tín hiệu phù hợp servo thực. | Bảng khoảng xung được datasheet hỗ trợ. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** servo nhận một xung nhưng làm sao quay tới vị trí?

**Bản chất.** Servo vị trí hobby thường gồm motor, truyền động, cảm biến vị trí và mạch điều khiển bên trong. Mạch so sánh vị trí yêu cầu với phản hồi và điều khiển motor để giảm sai lệch. Firmware của bạn gửi lệnh; servo ba dây thông thường không trả góc thực về bo.

Độ rộng xung ra lệnh vị trí trong phạm vi thiết bị hỗ trợ; nó khác bài PWM LED chỉ quan tâm duty. Thay chu kỳ mà giữ duty có thể thay độ rộng xung và làm thay lệnh. Không coi mọi servo đều hỗ trợ cùng khoảng xung hoặc đúng 180 độ.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T13-LT1:** Servo vị trí và motor DC trần khác nhau ở vòng phản hồi nào?

2. **T13-LT2:** Lệnh vị trí và vị trí thực khác nhau thế nào?

3. **T13-LT3:** Tại sao không điều khiển servo chỉ bằng phần trăm duty mà bỏ qua chu kỳ?

4. **T13-LT4:** Servo ba dây thông thường có báo góc thật về MCU không?

5. **T13-LT5:** Vì sao giữ tải đứng yên vẫn có thể tiêu thụ dòng?

6. **T13-LT6:** Vì sao phải thử vùng giữa và tải nhẹ trước toàn dải?

### E. Bài tập tính toán hoặc thiết kế

**T13-B1.** Trong bài toán tín hiệu giả định chu kỳ 20 ms, tính duty cho xung 1 ms, 1,5 ms, 2 ms. Không mặc định đây là dải servo của bạn.

**T13-B2.** Giữ duty 7,5%, đổi chu kỳ từ 20 ms xuống 10 ms. Xung thay đổi thế nào? Có thể coi lệnh vị trí giữ nguyên không?

### F. Bài thực hành theo trình tự

**TH1.** Vẽ ba luồng: nguồn nuôi motor; lệnh từ ESP32; phản hồi vị trí bên trong servo. Phân biệt lệnh với góc quan sát.

**TH2.** Cấp nguồn phù hợp, chung GND cho tín hiệu, thử không tải gần giữa với khoảng nhỏ.

**TH3.** Lập bảng ba giá trị xung và góc quan sát gần đúng. Xác định vùng làm việc tránh chạm giới hạn cơ khí.

**TH4.** Đi tới vị trí mới bằng các bước nhỏ theo thời gian; so với đổi lệnh một lần.

**Bài biến thể và kiểm tra sâu:** Ghi ba lệnh xung và góc quan sát, làm bản quét chậm trong giới hạn đã thử. Vẽ đường điện cấp motor tách với tín hiệu điều khiển.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T13-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T13-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Phân biệt điều khiển ngoài với phản hồi trong servo; không áp nhầm PWM LED; chuyển động không chạm chặn.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-14"></a>
## Tuần 14 — Mô men trọng tâm và quán tính

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 13. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Chọn đúng mô hình cho trục; có phép tính và phép thử, không chọn servo chỉ từ tên SG90.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [OpenStax — Torque and Equilibrium](https://openstax.org/books/college-physics-2e/pages/9-2-the-second-condition-for-equilibrium) | Đọc định nghĩa mô-men, cánh tay đòn và điều kiện cân bằng; tự làm lại ví dụ lực–khoảng cách. | Hình trục quay, lực, góc và trọng tâm. |
| 2 | [Adafruit — Servo Motors](https://learn.adafruit.com/adafruit-arduino-lesson-14-servo-motors) | Đối chiếu mô-men, điện áp và loại chuyển động của servo thực; xem hình lắp cơ cấu. | Bảng tải dự kiến và các thông số còn cần đo. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** một vật nhẹ có thể vẫn làm servo khó quay vì sao?

**Bản chất.** Mô-men lực phụ thuộc lực và khoảng cách vuông góc tới trục: độ lớn τ = F × r × sin(θ). Với trọng lực, trường hợp vuông góc cho τ = m × g × r. Công thức này không thay toàn bộ bài toán chuyển động: còn quán tính, gia tốc, ma sát và lực dây.

Nếu trục quay thẳng đứng lý tưởng, trọng lực không tạo mô-men quanh trục như cơ cấu nâng nằm ngang; tải vẫn gây lực lên gối đỡ và quán tính vẫn cản thay đổi tốc độ. Khối lượng xa trục thường làm quán tính quay lớn hơn. Torque ghi kgf·cm không phải khối lượng kg; phải xét cánh tay đòn và điều kiện thông số.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T14-LT1:** Khi nào được dùng τ = mgr, khi nào cần góc giữa lực và tay đòn?

2. **T14-LT2:** Trục quay đứng khác trục nâng ngang về ảnh hưởng trọng lực thế nào?

3. **T14-LT3:** Vì sao cùng khối lượng nhưng phân bố xa trục khó tăng tốc hơn?

4. **T14-LT4:** Ma sát và lực kéo dây xuất hiện ở đâu trong cơ cấu?

5. **T14-LT5:** Thông số kgf·cm có nghĩa gì, tại sao không đọc thành số kg nâng được ở mọi vị trí?

6. **T14-LT6:** Tại sao tốc độ và gia tốc khác nhau; tăng tốc nhanh có thể cần thêm mô-men ra sao?

### E. Bài tập tính toán hoặc thiết kế

**T14-B1.** Đầu 50 g, trọng tâm cách trục ngang 3 cm, trọng lực vuông góc tay đòn. Tính mô-men, dùng g = 9,81 m/s². Nếu khoảng cách 6 cm thì sao?

**T14-B2.** Trong mô hình điểm khối lượng I = mr², giữ m và tăng r gấp đôi. Quán tính thay đổi thế nào? Muốn cùng gia tốc góc, mô-men quán tính đổi ra sao?

### F. Bài thực hành theo trình tự

**TH1.** Giữ thước có tải nhẹ ở gần và xa bàn tay; mô tả cảm giác và liên hệ khoảng cách.

**TH2.** Trên giấy, tính mô-men trọng lực của đầu 50 g có trọng tâm cách trục ngang 3 cm: khoảng 0,0147 N·m ở tư thế cánh tay đòn vuông góc.

**TH3.** Làm đầu bìa, ghi trục thật là ngang hay đứng. Xác định tải nào tạo mô-men quanh trục, tải nào chủ yếu đè lên gối đỡ.

**TH4.** Thử chuyển động chậm trong dải nhỏ, rồi thay vị trí trọng tâm hoặc lực kéo dây một yếu tố mỗi lần. Không giữ cơ cấu kẹt.

**Bài biến thể và kiểm tra sâu:** Làm đầu bìa và ghi trục thật. Thay vị trí trọng tâm hoặc giảm dây kéo, giữ các yếu tố khác rồi thử lại. Vẽ cơ cấu để người khác thấy vì sao thay đổi có ích.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T14-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T14-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Chọn đúng mô hình cho trục; có phép tính và phép thử, không chọn servo chỉ từ tên SG90.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-15"></a>
## Tuần 15 — Nguồn có trở kháng và tải thay đổi theo thời gian

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 14. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Có sơ đồ dòng đi/về; quyết định sửa từ bằng chứng; biết giới hạn của phép đo DMM.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [All About Circuits — Direct Current](https://www.allaboutcircuits.com/textbook/direct-current/) | Đọc Power in Electric Circuits, Battery Ratings và Practical Considerations — Batteries; tập trung nguồn dưới tải. | Ngân sách dòng/công suất theo chế độ. |
| 2 | [Adafruit — Servo Motors](https://learn.adafruit.com/adafruit-arduino-lesson-14-servo-motors) | Đọc phần nguồn và hiện tượng servo ảnh hưởng bo. | Sơ đồ đường cấp và đường hồi dòng. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** vì sao chỉ lúc servo chạy bo mới reset?

**Bản chất.** Nguồn và đường dây thực có điện trở/trở kháng. Khi dòng tăng, sụt áp trên đường cấp tăng. Xấp xỉ DC ΔV = I × R giúp hình dung: 0,5 A qua đường tổng 0,4 Ω tạo sụt 0,2 V. Tải motor thay đổi nhanh còn liên quan đáp ứng nguồn, dây và tụ; chỉ nhìn giá trị danh định không đủ.

GND vừa là mốc tín hiệu vừa nằm trên đường dòng hồi về. Dòng motor qua đoạn dây chung có thể làm mốc tham chiếu thay đổi. Phân nhánh đường công suất và đường tín hiệu hợp lý giúp giảm ảnh hưởng, nhưng vẫn cần GND chung phù hợp cho tín hiệu không cách ly.

Brownout/reset khi áp thấp là một triệu chứng; code chưa đổi không có nghĩa phần mềm phải đúng hoặc nguồn chắc chắn lỗi. Cần số đo, reset reason và thử cô lập.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T15-LT1:** Dòng trung bình và dòng đỉnh khác nhau thế nào?

2. **T15-LT2:** Vì sao điện áp tại đầu nguồn khác điện áp tại tải?

3. **T15-LT3:** GND có luôn cùng điện thế tại mọi điểm trong mạch thực không?

4. **T15-LT4:** Vì sao reset khi servo chạy là triệu chứng chứ chưa là bằng chứng nguồn lỗi?

5. **T15-LT5:** Tụ giúp ở thang thời gian nào; khi nào phải sửa nguồn/dây?

6. **T15-LT6:** Cần kiểm tra gì trước khi dùng USB và nguồn servo riêng?

### E. Bài tập tính toán hoặc thiết kế

**T15-B1.** Tổng điện trở cả đường cấp và hồi là 0,4 Ω, dòng 0,8 A. Tính sụt áp và công suất mất trên dây. Nguồn 5 V thì tải gần bao nhiêu theo mô hình này?

**T15-B2.** Một hệ 5 V giả định: bo 0,25 A, OLED 0,03 A, audio 0,40 A, servo đỉnh 0,80 A. Cộng dòng đồng thời; thêm biên thiết kế 30% cho bài tính. Vì sao đây chưa là chọn nguồn cuối?

### F. Bài thực hành theo trình tự

**TH1.** Vẽ đầy đủ cả đường đi và về của dòng cho bo, OLED, servo. Không nối song song hai đầu ra nguồn nếu chưa có thiết kế cho việc đó.

**TH2.** Ghi điện áp tại nguồn và gần tải lúc nghỉ; kiểm tra đầu nối/dây.

**TH3.** Thử màn hình riêng, servo riêng, kết hợp; ghi reset và điều kiện tải. Đồng hồ chỉ cho một phần bằng chứng.

**TH4.** Theo giả thuyết, thay một thứ: giảm tải, cải thiện dây hoặc nguồn phù hợp. Chạy lại 20 chuyển động nhẹ.

**Bài biến thể và kiểm tra sâu:** Thử riêng từng tải rồi kết hợp; ghi Vs ở nguồn và gần bo, reset reason. Cải thiện đúng một yếu tố, chạy 20 lần chuyển động để so trước/sau.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T15-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T15-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Có sơ đồ dòng đi/về; quyết định sửa từ bằng chứng; biết giới hạn của phép đo DMM.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-16"></a>
## Tuần 16 — Máy trạng thái làm hành vi có thể lý giải

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 15. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Không có đường chuyển mơ hồ; mọi state hoạt động có đường hủy/lỗi; kết quả cũ không gây hành động.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [Adafruit — Multi-tasking Part 1](https://learn.adafruit.com/multi-tasking-the-arduino-part-1/a-classy-solution) | Đọc A classy solution và All together now; chỉ lấy cách lưu trạng thái/điều phối. | Một bảng trạng thái không phụ thuộc ngôn ngữ. |
| 2 | [Gemini — Structured outputs](https://ai.google.dev/gemini-api/docs/structured-output) | Đọc khái niệm schema ở mức hình dạng thông điệp; chưa cần gọi AI. | Tập tên biểu cảm/chuyển động được phép. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** khi vừa có nút nhấn vừa có phản hồi mạng, robot nên làm gì trước?

**Bản chất.** Máy trạng thái biểu diễn quy tắc: trạng thái hiện tại + sự kiện + điều kiện → trạng thái mới + hành động. Trạng thái mô tả điều đang được phép xảy ra; sự kiện mô tả việc vừa xảy ra. Nếu quy tắc chỉ nằm rải rác ở nhiều hàm, dễ phát âm khi đang nghe hoặc xử lý phản hồi cũ.

Tách quyết định khỏi driver: bộ điều phối yêu cầu “mắt chờ” và “ngừng phát”, driver chuyển chúng thành thao tác phần cứng. Nhờ vậy có thể kiểm tra quy tắc trên laptop trước khi đấu dây.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T16-LT1:** State, event, guard và action là gì? Cho ví dụ của robot.

2. **T16-LT2:** Vì sao nhiều cờ boolean dễ tạo trạng thái mâu thuẫn?

3. **T16-LT3:** Hành vi nào được phép khi đang Listening/Thinking/Speaking?

4. **T16-LT4:** Khi hủy, cần dọn những tài nguyên và bỏ những kết quả nào?

5. **T16-LT5:** Vì sao logic điều phối nên tách driver?

6. **T16-LT6:** Có thể kiểm thử phần nào trên laptop mà không cần bo?

### E. Bài tập tính toán hoặc thiết kế

**T16-B1.** Có 3 cờ listening/thinking/speaking. Có bao nhiêu tổ hợp boolean? Nếu chỉ cho phép không làm gì hoặc đúng một việc, bao nhiêu tổ hợp hợp lệ?

**T16-B2.** Phân tích chuỗi Start → StopRecording → Cancel → Result(old) → Start. Ghi state và hành động sau từng event.

### F. Bài thực hành theo trình tự

**TH1.** Viết bảng đầy đủ, gồm sự kiện không hợp lệ và hành vi bỏ qua/báo lỗi.

**TH2.** Giả lập tất cả trên laptop bằng sự kiện chữ, chưa gọi AI.

**TH3.** Gắn mắt và nút; trạng thái Nói chỉ dùng thời gian giả.

**TH4.** Thử nhấn nhanh, hủy rồi có kết quả muộn, nhận kết quả hai lần.

Bảng khởi đầu để mở rộng thành bảng chuyển trạng thái đầy đủ:

| Trạng thái | Sự kiện hợp lệ | Kết quả |
|---|---|---|
| Chờ | Bắt đầu | Nghe |
| Nghe | Kết thúc thu | Suy nghĩ |
| Suy nghĩ | Phản hồi đúng phiên | Nói |
| Nói | Phát xong | Chờ |
| Bất kỳ trạng thái hoạt động | Hủy | Dọn tài nguyên rồi Chờ |
| Hoạt động | Lỗi không xử lý được tại chỗ | Lỗi có đường quay lại |

**Bài biến thể và kiểm tra sâu:** Viết bảng đầy đủ state/event/điều kiện/next/action cho 5 trạng thái; thử tối thiểu 10 chuỗi trên laptop, gồm sự kiện sai thứ tự và trùng.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T16-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T16-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Không có đường chuyển mơ hồ; mọi state hoạt động có đường hủy/lỗi; kết quả cũ không gây hành động.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-17"></a>
## Tuần 17 — Đồng thời hàng đợi và giới hạn tài nguyên

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 16. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Nói được giới hạn tài nguyên và lý do thiết kế; demo vẫn hoạt động khi có việc chậm.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [ESP-IDF FreeRTOS](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/system/freertos_idf.html) | Đọc Tasks, scheduling, queue và mutex ở mức vai trò; dùng mục lục đến phần tương ứng, không đọc mọi API. | Sơ đồ task và nơi sở hữu từng tài nguyên. |
| 2 | [Adafruit — Multi-tasking Part 1](https://learn.adafruit.com/multi-tasking-the-arduino-part-1/a-classy-solution) | Ôn Ditch the delay và Using millis; đối chiếu chờ trong một luồng với task được nhường lịch. | So sánh cooperative loop và nhiều task. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** làm nhiều việc cùng lúc có nghĩa CPU phải thực hiện mọi lệnh cùng một thời điểm không?

**Bản chất.** Đồng thời là tổ chức nhiều công việc có tiến độ xen kẽ; song song là thực thi tại cùng thời điểm trên tài nguyên khác nhau. Một hàm blocking có thể giữ luồng chờ; nhiều task không tự bảo đảm phản hồi nhanh nếu cùng tranh tài nguyên hoặc giữ khóa quá lâu.

Hàng đợi tách nơi tạo sự kiện và nơi xử lý. Nếu tốc độ tạo lâu dài lớn hơn tốc độ xử lý, hàng đợi không giới hạn sẽ tăng mãi. Hàng đợi hữu hạn buộc chọn chính sách: bỏ bản cũ, từ chối bản mới, chờ hoặc báo lỗi. Với biểu cảm mắt, giữ trạng thái mới nhất có thể hợp lý; với dữ liệu âm thanh liên tục, bỏ tùy tiện sẽ làm hỏng tiếng.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T17-LT1:** Concurrency khác parallelism thế nào?

2. **T17-LT2:** Blocking một task có luôn chặn toàn bộ hệ RTOS không? Điều kiện nào khiến chức năng khác vẫn bị ảnh hưởng?

3. **T17-LT3:** Queue giải quyết điều gì và không giải quyết điều gì?

4. **T17-LT4:** Queue đầy nên bỏ/chờ/từ chối thế nào cho mắt và cho audio?

5. **T17-LT5:** Deadlock, starvation và timeout khác nhau ra sao?

6. **T17-LT6:** Watchdog phát hiện được gì, chưa chứng minh được gì?

### E. Bài tập tính toán hoặc thiết kế

**T17-B1.** Producer 120 sự kiện/s, consumer 100 sự kiện/s, queue ban đầu rỗng chứa 40 sự kiện. Theo tốc độ trung bình cố định, sau bao lâu đầy?

**T17-B2.** Nút cần được xử lý trong 100 ms nhưng task đang giữ mutex cần thiết trong 2 s. Thêm core có tự bảo đảm mục tiêu không? Đề xuất thiết kế khác.

### F. Bài thực hành theo trình tự

**TH1.** Liệt kê bốn việc và thời gian phản hồi mong muốn: nút, mắt, chuyển động, yêu cầu mạng. Gán một nơi sở hữu mỗi tài nguyên.

**TH2.** Giả lập mạng mất 10 giây trong khi mắt/nút vẫn chạy.

**TH3.** Dùng hàng đợi có giới hạn nhỏ để dễ thử đầy. Gửi nhanh nhiều sự kiện và quan sát chính sách đã chọn.

**TH4.** Thử hủy, timeout, phản hồi trễ; thêm mã phiên để phân biệt kết quả cũ.

**Bài biến thể và kiểm tra sâu:** Giả lập mạng chậm 10 s, queue đầy và hủy. Ghi latency nút và queue length; dùng queue hữu hạn, ghi rõ chính sách lúc đầy.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T17-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T17-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Nói được giới hạn tài nguyên và lý do thiết kế; demo vẫn hoạt động khi có việc chậm.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-18"></a>
## Tuần 18 — Mạng là nhiều lớp độc lập

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 17. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Tự giải thích một request đi đâu; không sửa tầng AI khi kết nối cơ bản chưa chạy.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [MDN — How does the Internet work?](https://developer.mozilla.org/en-US/docs/Learn_web_development/Howto/Web_mechanics/How_does_the_Internet_work) | Đọc toàn bài, tập trung router, địa chỉ và DNS. | Sơ đồ mạng có địa chỉ và hướng gửi. |
| 2 | [MDN — Client-server overview](https://developer.mozilla.org/en-US/docs/Learn_web_development/Extensions/Server-side/First_steps/Client-Server_overview) | Đọc Requests, Responses và ví dụ client/server. | Phân biệt server trên laptop với dịch vụ AI ở xa. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** vì sao đã kết nối Wi-Fi vẫn không gọi được máy chủ?

**Bản chất.** Wi-Fi cung cấp kết nối liên kết; IP giúp định địa chỉ/định tuyến; TCP vận chuyển luồng byte có thứ tự theo kết nối; HTTP đặt quy tắc yêu cầu/phản hồi phía trên. DNS đổi tên thành địa chỉ. Mỗi lớp thành công không chứng minh lớp trên cũng thành công.

Cổng giúp chọn dịch vụ trên một máy. localhost hoặc loopback chỉ chính thiết bị đang dùng nó. Laptop có thể mở dịch vụ chỉ trên loopback nên ESP32 cùng Wi-Fi vẫn không truy cập được. Hai thiết bị cùng tên Wi-Fi cũng có thể bị client isolation hoặc firewall chặn.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T18-LT1:** Wi-Fi, IP, TCP, HTTP và DNS đảm nhận phần nào?

2. **T18-LT2:** Vì sao cùng Wi-Fi không bảo đảm hai thiết bị truy cập được nhau?

3. **T18-LT3:** localhost nghĩa là gì trên laptop và trên ESP32?

4. **T18-LT4:** Port phân biệt điều gì?

5. **T18-LT5:** Server bind vào loopback khác lắng nghe địa chỉ mạng thế nào?

6. **T18-LT6:** Cách phân biệt sai IP, sai port, firewall và ứng dụng lỗi?

### E. Bài tập tính toán hoặc thiết kế

**T18-B1.** Laptop có IP LAN 192.168.1.20 và server cổng 8080. Robot gọi http://127.0.0.1:8080/health. Sai ở đâu và URL LAN dự kiến là gì?

**T18-B2.** Một lỗi xuất hiện trước khi có HTTP response. Có thể kết luận HTTP status là 500 không? Lập các tầng kiểm tra.

### F. Bài thực hành theo trình tự

**TH1.** Vẽ ESP32 → router → laptop; ghi IP, cổng server và phần nào do DHCP cấp.

**TH2.** Tạo endpoint /health trên laptop. Gọi tại laptop, sau đó từ thiết bị khác trong LAN với cấu hình lắng nghe phù hợp.

**TH3.** Cho ESP32 gọi và log lỗi theo bước kết nối/gửi/đọc.

**TH4.** Thử sai IP, sai cổng và tắt server riêng biệt. Giải thích vì sao chúng có thể cùng biểu hiện “không có phản hồi” nhưng nguyên nhân khác.

**Bài biến thể và kiểm tra sâu:** Tạo /health, gọi từ laptop rồi thiết bị LAN rồi ESP32. Tắt server/sai port/ngắt mạng riêng từng ca và lưu loại lỗi.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T18-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T18-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Tự giải thích một request đi đâu; không sửa tầng AI khi kết nối cơ bản chưa chạy.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-19"></a>
## Tuần 19 — Giao thức là một thỏa thuận về ý nghĩa

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 18. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Có schema và ví dụ cả hai chiều; phân biệt lỗi truyền, parse, schema và logic.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [MDN — HTTP Overview](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Overview) | Đọc HTTP messages, request/response, headers và body; đọc ví dụ hoàn chỉnh. | Chú thích một request và response. |
| 2 | [Gemini — Structured outputs](https://ai.google.dev/gemini-api/docs/structured-output) | Đọc schema và các kiểu dữ liệu; tập trung giá trị được phép và trường bắt buộc. | Hợp đồng JSON của robot. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** nhận đúng byte đã đủ để robot hành động đúng chưa?

**Bản chất.** TCP không gắn sẵn ranh giới “một thông điệp JSON” cho mỗi lần đọc. HTTP hoặc cơ chế framing phải giúp xác định dữ liệu đầy đủ. JSON hợp cú pháp cũng chưa chắc đúng schema: tên trường, kiểu dữ liệu, giá trị cho phép và phiên bản có thể sai.

HTTP status nói về xử lý yêu cầu ở tầng giao thức; phản hồi 200 không thay thế việc kiểm tra nội dung. Firmware phải kiểm tra biểu cảm/chuyển động nằm trong tập cho phép. AI hoặc server không được trực tiếp tùy ý điều khiển điện áp và chân GPIO.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T19-LT1:** TCP truyền luồng byte hay các thông điệp JSON nguyên vẹn?

2. **T19-LT2:** Vì sao parse JSON thành công chưa chứng minh yêu cầu hợp lệ?

3. **T19-LT3:** Header, body và status có vai trò khác nhau thế nào?

4. **T19-LT4:** request_id, user_id, device_id có thể thay thế nhau không?

5. **T19-LT5:** Vì sao phải giới hạn kích thước body và độ dài text?

6. **T19-LT6:** Khi firmware và server khác phiên bản giao thức, cần xử lý thế nào?

### E. Bài tập tính toán hoặc thiết kế

**T19-B1.** Schema yêu cầu protocol_version số nguyên 1, request_id chuỗi, expression trong {idle, happy, error}. Phân loại: thiếu request_id; version='1'; expression='dance'; đúng tất cả.

**T19-B2.** Lần đọc TCP thứ nhất chứa nửa JSON, lần thứ hai chứa phần còn lại và đầu thông điệp kế. Vì sao parse từng lần đọc riêng là sai? Nêu cơ chế phân khung.

### F. Bài thực hành theo trình tự

**TH1.** Định nghĩa thông điệp có protocol_version, request_id, expression. Ghi kiểu dữ liệu và ba giá trị expression được phép.

**TH2.** Server trả biểu cảm cố định, ESP32 hiển thị. Giữ việc hiểu thông điệp tách khỏi driver OLED.

**TH3.** Thử JSON thiếu trường, sai kiểu, giá trị lạ, response body quá lớn và dữ liệu thiếu. Đặt giới hạn kích thước.

**TH4.** Đổi version giả để kiểm tra từ chối rõ ràng; ghi ví dụ response thành công và lỗi.

**Bài biến thể và kiểm tra sâu:** Viết 10 payload:3hợp lệ,7sai theo các kiểu khác nhau. Chạy qua parser và bộ kiểm tra ngữ nghĩa; phản hồi phải hữu hạn, rõ loại lỗi và không gây chuyển động tùy ý.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T19-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T19-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Có schema và ví dụ cả hai chiều; phân biệt lỗi truyền, parse, schema và logic.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-20"></a>
## Tuần 20 — Timeout retry và tác động lặp

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 19. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Không retry vô hạn; có đường phục hồi; chứng minh không lặp tác động với cùng yêu cầu theo phạm vi thiết kế.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [MDN — HTTP methods](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Methods) | Đọc đặc tính safe và idempotent của phương thức; hiểu ý nghĩa, không suy rằng mọi server tự đúng. | Ví dụ đọc hồ sơ và lệnh gây hành động. |
| 2 | [MDN — HTTP status codes](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status) | Đọc nhóm 2xx/4xx/5xx, 400/401/403/404/409/429/500/503. | Bảng lỗi nào sửa input, lỗi nào có thể cân nhắc retry. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** hết thời gian chờ có chứng minh máy chủ chưa làm việc không?

**Bản chất.** Client có thể timeout sau khi server đã thực hiện nhưng phản hồi chưa tới. Retry một thao tác gây hiệu ứng có thể thực hiện lại thao tác. Đây là bài toán ý nghĩa hành động, không chỉ mạng. Idempotency nghĩa là việc lặp cùng yêu cầu không gây thêm hiệu ứng ngoài kết quả một lần theo quy tắc thiết kế.

Khoảng chờ tăng dần tránh gửi dồn khi dịch vụ lỗi; cần giới hạn số lần và thời gian tổng. Hủy ở client không tự đảm bảo dịch vụ từ xa ngừng tính toán hoặc hoàn phí. HTTPS bảo vệ kênh theo TLS khi kiểm tra đúng chứng chỉ; xác thực thiết bị và phân quyền là các yêu cầu riêng.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T20-LT1:** Client timeout có chứng minh server chưa thực hiện không?

2. **T20-LT2:** Retry một lệnh chuyển động có nguy cơ gì?

3. **T20-LT3:** Idempotency khác chỉ dùng cùng HTTP method thế nào?

4. **T20-LT4:** Backoff và giới hạn retry để làm gì?

5. **T20-LT5:** Hủy client có bảo đảm server dừng hoặc không tính phí không?

6. **T20-LT6:** HTTPS, kiểm tra chứng chỉ, xác thực và phân quyền khác nhau thế nào?

### E. Bài tập tính toán hoặc thiết kế

**T20-B1.** Server thực hiện lệnh ở t=2 s, response tới client t=6 s; client timeout t=5 s rồi gửi lại. Vẽ timeline và nêu cách tránh hành động lặp.

**T20-B2.** Bài thử tối đa 3 lần gửi, mỗi lần có giới hạn tổng 5 s; đợi 1 s trước lần 2 và 2 s trước lần 3. Tính giới hạn chuỗi này bỏ overhead.

### F. Bài thực hành theo trình tự

**TH1.** Vẽ ba mốc gửi → server thực hiện → client nhận; đánh dấu nơi có thể mất phản hồi.

**TH2.** Server cố tình chậm hơn timeout. Quan sát cả log server và client để chứng minh server có thể đã xử lý.

**TH3.** Gửi lại cùng request_id và xử lý trùng theo quy tắc. Một yêu cầu “gật đầu” bị gửi trùng không gây gật lặp ngoài ý muốn.

**TH4.** Thử mất Wi-Fi, bật lại, hủy trong lúc retry; log số lần và kết quả cuối. Giữ khóa AI ở server.

**Bài biến thể và kiểm tra sâu:** Dùng server giả chậm/lỗi/trùng để thử. Ghi request_id, attempt, thời gian, kết quả và hành vi robot. Mất mạng robot vẫn nhận thao tác hủy.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T20-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T20-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Không retry vô hạn; có đường phục hồi; chứng minh không lặp tác động với cùng yêu cầu theo phạm vi thiết kế.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-21"></a>
## Tuần 21 — Mô hình ngôn ngữ và đầu vào thực tế của nó

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 20. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Biết yêu cầu gửi thật; tách câu trả lời khỏi lệnh phần cứng; có bộ đánh giá thay vì chọn một câu demo đẹp.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [Gemini — Text generation](https://ai.google.dev/gemini-api/docs/text-generation) | Đọc Text generation, System instructions, Multi-turn và ví dụ REST; tập trung cấu trúc request. | Lưu bản request đã loại khóa để thấy AI nhận gì. |
| 2 | [Gemini — Structured outputs](https://ai.google.dev/gemini-api/docs/structured-output) | Đọc structured response và xử lý schema; phân biệt định dạng với tính đúng nội dung. | Schema text/expression hữu hạn. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** mỗi lần gọi, mô hình thật sự biết những gì về người đang nói?

**Bản chất.** Ứng dụng gửi thông tin được chọn vào một yêu cầu; mô hình sinh đầu ra dựa trên ngữ cảnh được cung cấp và khả năng đã học trong huấn luyện. Một câu trả lời tự tin không phải bằng chứng đúng sự thật. Huấn luyện mô hình khác với việc dùng API để suy luận.

System instruction mô tả cách trả lời; lịch sử và hồ sơ cung cấp ngữ cảnh. Giới hạn token là giới hạn theo đơn vị xử lý của mô hình, không đơn giản bằng số từ tiếng Việt. Lịch sử càng dài thường tăng dữ liệu cần xử lý, chi phí và khả năng đưa thông tin không còn liên quan.

Structured output giúp ép hình dạng dữ liệu theo tính năng API, nhưng vẫn cần kiểm tra giá trị và xử lý lỗi/từ chối. Đừng dùng câu văn tự do để quyết định trực tiếp GPIO.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T21-LT1:** Inference khác training và fine-tuning thế nào?

2. **T21-LT2:** Mô hình nhìn thấy thông tin nào trong một lượt gọi?

3. **T21-LT3:** System instruction, lịch sử và hồ sơ khác nhau ở vai trò gì?

4. **T21-LT4:** Vì sao câu trả lời tự tin không phải bằng chứng sự thật?

5. **T21-LT5:** Token khác từ và ký tự thế nào về cách dùng thực tế?

6. **T21-LT6:** Đầu ra đúng JSON có thể vẫn sai nội dung hoặc không phù hợp trạng thái robot ra sao?

### E. Bài tập tính toán hoặc thiết kế

**T21-B1.** A:không gửi lịch sử; B:gửi 3 lượt gần nhất; C:gửi 3 lượt và hồ sơ tên. Thiết kế 5 câu thử để phân biệt ba cấu hình, giữ model và chỉ dẫn giống nhau.

**T21-B2.** Có 10 câu, cấu hìnhA đáp ứng mục tiêu 7 câu, B 9 câu nhưng B dài hơn gấp đôi. Kết luận nào được và chưa được phép rút ra?

### F. Bài thực hành theo trình tự

**TH1.** Gọi một câu tiếng Việt từ laptop; lưu bản yêu cầu đã loại khóa bí mật và phản hồi.

**TH2.** Thêm chỉ dẫn “tối đa ba câu, xưng mình–bạn”; thay duy nhất chỉ dẫn để so sánh.

**TH3.** Tạo danh sách 10 câu gồm câu thiếu thông tin và yêu cầu sai; đánh giá trả lời đúng mục tiêu, độ dài và cách xử lý điều chưa biết.

**TH4.** Cho đầu ra gồm text và expression theo schema; kiểm tra lại expression ở server/firmware trước khi dùng.

**Bài biến thể và kiểm tra sâu:** Làm chatbot chữ, 10 câu cố định, thay một chỉ dẫn mỗi lần. Ghi kết quả trung gian và dùng schema cho biểu cảm; đặt giới hạn output và ngân sách thử.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T21-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T21-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Biết yêu cầu gửi thật; tách câu trả lời khỏi lệnh phần cứng; có bộ đánh giá thay vì chọn một câu demo đẹp.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-22"></a>
## Tuần 22 — Dữ liệu bền vững và định danh

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 21. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Làm được CRUD và truy vấn đúng người; không phụ thuộc lời nói của AI để biết dữ liệu đã được lưu.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [SQLBolt — Bài học SQL có bài tập tương tác](https://sqlbolt.com/) | Làm các bài SELECT, filtering, JOIN, INSERT, UPDATE, DELETE và CREATE TABLE trong mục lục; đọc bài rồi tự giải bài tương tác. | Lưu câu SQL tự viết và lỗi đã sửa. |
| 2 | [SQLite — SQL Language](https://www.sqlite.org/lang.html) | Đọc CREATE TABLE, SELECT, INSERT, UPDATE, DELETE, BEGIN TRANSACTION. | Lược đồ và thao tác SQLite đúng user_id. |
| 3 | [SQLite — Command Line Shell](https://www.sqlite.org/cli.html) | Đọc Getting Started và cách mở database file. | Tự tạo và mở lại file dữ liệu. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** robot nhớ được tên sau khi tắt máy bằng cơ chế nào?

**Bản chất.** Biến trong RAM mất khi tiến trình kết thúc; dữ liệu ghi đúng vào lưu trữ bền vững có thể được đọc lại. Cơ sở dữ liệu quản lý cấu trúc, truy vấn và tính nhất quán theo cơ chế của nó. user_id ổn định giúp gắn thông tin với người đúng; tên hiển thị không phải khóa tốt vì có thể trùng hoặc đổi.

Một bảng hồ sơ lưu tên/cách xưng hô; bảng sở thích có thể có nhiều hàng cho một người. Ghi nguồn và thời điểm cập nhật giúp phân biệt người dùng xác nhận với mô hình suy đoán. Transaction gom các thay đổi liên quan để tránh trạng thái nửa chừng.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T22-LT1:** RAM và dữ liệu bền vững khác nhau thế nào?

2. **T22-LT2:** Vì sao tên người dùng không thích hợp làm khóa định danh duy nhất?

3. **T22-LT3:** Một người có nhiều sở thích nên lưu quan hệ ra sao?

4. **T22-LT4:** Primary key, foreign key và điều kiện WHERE phục vụ gì?

5. **T22-LT5:** Transaction tránh trạng thái nửa chừng thế nào?

6. **T22-LT6:** Vì sao phải ghi nguồn/xác nhận/thời điểm của thông tin?

### E. Bài tập tính toán hoặc thiết kế

**T22-B1.** Thiết kế users và preferences cho A có 2 sở thích, B có 1 sở thích. Viết SQL thêm, liệt kê sở thích A, đổi tên A, xóa đúng 1 sở thích A.

**T22-B2.** Hai bước 'xóa sở thích cũ' rồi 'thêm sở thích mới', ứng dụng lỗi giữa hai bước. Mô tả dữ liệu nếu không transaction và thiết kế nguyên tử.

### F. Bài thực hành theo trình tự

**TH1.** Vẽ bảng users và preferences, quan hệ bằng user_id; ghi trường nào bắt buộc.

**TH2.** Thêm, đọc, sửa và xóa một hồ sơ thử trực tiếp bằng SQL, luôn chỉ đúng user_id trong điều kiện.

**TH3.** Cho server đọc hồ sơ trước gọi AI; restart server và đọc lại.

**TH4.** Thử đổi tên nhưng giữ user_id, thêm hai sở thích rồi xóa một. Nếu lưu nhiều thay đổi liên quan, dùng transaction theo thư viện đã chọn.

**Bài biến thể và kiểm tra sâu:** Thao tác với dữ liệu giả trên CLI trước ghép server. Restart server, kiểm tra còn hồ sơ. Thử tên trùng nhưng ID khác để chứng minh không lẫn người.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T22-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T22-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Làm được CRUD và truy vấn đúng người; không phụ thuộc lời nói của AI để biết dữ liệu đã được lưu.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-23"></a>
## Tuần 23 — Bộ nhớ AI là quy trình chọn thông tin

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 22. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Cá nhân hóa có nguồn và sửa/quên kiểm chứng được; không nhầm lưu trữ với trí nhớ tự động của model.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [Gemini — Text generation](https://ai.google.dev/gemini-api/docs/text-generation) | Ôn multi-turn, xem chính xác lịch sử được gửi ở đâu. | Sơ đồ xây context cho mỗi request. |
| 2 | [SQLite — SQL Language](https://www.sqlite.org/lang.html) | Ôn SELECT, UPDATE, DELETE và transaction theo mô hình tuần22. | Danh sách mọi nơi chứa thông tin người dùng. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** vì sao đã xóa một sở thích trong bảng mà robot vẫn nhắc lại?

**Bản chất.** Một thông tin có thể tồn tại trong hồ sơ, lịch sử hội thoại, tóm tắt hoặc cache. Xóa một bản mà vẫn gửi bản khác vào yêu cầu thì mô hình tiếp tục nhìn thấy nó. Muốn quên trong phạm vi ứng dụng cần quản lý các nơi ứng dụng lưu và sử dụng thông tin; không tuyên bố xóa được mọi hệ thống bên ngoài chỉ bằng một truy vấn local.

Danh tính cần được chọn trước truy xuất. Bản đầu có thể chọn người bằng cấu hình/nút trên giao diện server, chưa cần nhận dạng giọng nói. Cá nhân hóa tốt dựa trên thông tin có nguồn và được xác nhận, không âm thầm biến suy đoán thành sự thật.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T23-LT1:** Hồ sơ, lịch sử, tóm tắt và cache khác nhau thế nào?

2. **T23-LT2:** Vì sao xóa một hàng hồ sơ vẫn có thể khiến AI nhắc lại thông tin?

3. **T23-LT3:** Định danh người dùng phải xảy ra trước bước nào?

4. **T23-LT4:** Thông tin người dùng nói trực tiếp và suy đoán của mô hình nên lưu khác nhau ra sao?

5. **T23-LT5:** Nếu hồ sơ mới mâu thuẫn lịch sử cũ, ứng dụng chọn nguồn nào và vì sao?

6. **T23-LT6:** Phạm vi 'quên' của ứng dụng khác việc xóa dữ liệu ở nhà cung cấp ngoài thế nào?

### E. Bài tập tính toán hoặc thiết kế

**T23-B1.** A từng thích trà, vừa xác nhận thích cà phê; tóm tắt cũ vẫn ghi trà. Viết quy tắc cập nhật và request tiếp theo để tránh mâu thuẫn.

**T23-B2.** Thiết kế 12 ca kiểm thử:3 đúng người,3 sửa,3 quên,3 không có thông tin. Mỗi ca ghi dữ liệu trước, thao tác, request mong muốn và câu trả lời có thể chấp nhận.

### F. Bài thực hành theo trình tự

**TH1.** Vẽ các nơi đang chứa thông tin người dùng và đường đi của chúng vào request.

**TH2.** Tạo A thích trà, B thích cà phê; luân phiên user_id và lưu ngữ cảnh gửi đi để kiểm tra không lẫn.

**TH3.** A đổi từ trà sang nước lọc; xác nhận rồi cập nhật. Kiểm tra cả hồ sơ và ngữ cảnh lượt kế.

**TH4.** A yêu cầu quên sở thích. Xóa/đánh dấu vô hiệu các bản dẫn xuất liên quan và thử cuộc hội thoại mới.

**Bài biến thể và kiểm tra sâu:** Chạy A/B xen kẽ, sửa/xóa rồi restart. Lưu bản context đã loại bí mật để chứng minh thông tin được chọn đúng.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T23-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T23-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Cá nhân hóa có nguồn và sửa/quên kiểm chứng được; không nhầm lưu trữ với trí nhớ tự động của model.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-24"></a>
## Tuần 24 — Âm thanh là tín hiệu được lấy mẫu

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 23. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Không nhầm byte với mẫu; tự xác định format; biết phần nào cần đổi để API và I²S hiểu đúng.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [Audacity — Digital Audio Fundamentals](https://manual.audacityteam.org/man/digital_audio.html) | Đọc toàn bài, chú ý sampling, sample format/size, clipping; tự giải thích các hình sóng. | Bảng sample rate/bit depth/channel/encoding/container. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** một file âm thanh đang lưu điều gì, và vì sao sai thông số phát làm giọng nhanh hoặc chậm?

**Bản chất.** Micro biến dao động áp suất thành tín hiệu; hệ số hóa lấy mẫu theo thời gian và lượng tử hóa biên độ. Sample rate là số mẫu mỗi giây; bit depth xác định số mức biểu diễn mỗi mẫu theo định dạng. Mono có một kênh, stereo có hai. PCM là cách biểu diễn mẫu; WAV là dạng container có thể chứa PCM và thông tin định dạng, không đồng nghĩa mọi WAV đều cùng codec.

Đọc 16.000 mẫu mỗi giây khác đọc 16.000 byte mỗi giây. PCM mono 16 kHz/16 bit cần 32.000 byte/s. Nếu ghi mẫu ở tốc độ này nhưng diễn giải phát theo tốc độ khác, thời lượng và cao độ có thể sai. Điều kiện Nyquist lý tưởng yêu cầu tần số lấy mẫu lớn hơn hai lần tần số cao nhất cần biểu diễn, với hệ thực cần lọc chống aliasing. Không cần chứng minh Fourier để làm bài đầu.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T24-LT1:** Micro, ADC, mẫu số và file nằm ở các tầng nào?

2. **T24-LT2:** Sample rate khác bit depth và số kênh thế nào?

3. **T24-LT3:** PCM khác WAV và âm thanh nén thế nào?

4. **T24-LT4:** Nyquist nói điều gì và vì sao còn cần lọc chống aliasing?

5. **T24-LT5:** Clipping khác âm lượng nhỏ hoặc nhiễu nền thế nào?

6. **T24-LT6:** Sai tốc độ diễn giải mẫu làm thời lượng/cao độ đổi thế nào?

### E. Bài tập tính toán hoặc thiết kế

**T24-B1.** Tính dữ liệu PCM mono 16 kHz 16 bit cho 5 s và 10 s. Tính 5 s nếu lưu mỗi mẫu trong 32 bit. Tính 5 s stereo 16 bit.

**T24-B2.** File có 160.000mẫu một kênh, thu 16 kHz nhưng phát như 8 kHz. Thời lượng gốc và phát? Cao độ lý tưởng thay đổi ra sao nếu chỉ đổi tốc độ diễn giải?

### F. Bài thực hành theo trình tự

**TH1.** Thu câu ngắn trên laptop, nhìn waveform và phân biệt im lặng/nói.

**TH2.** Xuất PCM WAV mono 16 kHz/16 bit, kiểm tra thuộc tính và nghe lại.

**TH3.** Tính 5 s = 160.000 byte dữ liệu; so file thực có header. Tính lại với 32 bit/mẫu và hai kênh.

**TH4.** Tạo bản sao, thử gain vừa phải rồi quá lớn để nhìn clipping; giảm âm lượng phát để bảo vệ tai. So sánh độ rõ chứ không chỉ độ to.

**Bài biến thể và kiểm tra sâu:** Thu bằng laptop, xuất một WAV chuẩn, xem thông tin và dạng sóng. Tạo bản sao để thử gain/clipping, không phá file gốc; tính dung lượng rồi so thực.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T24-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T24-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Không nhầm byte với mẫu; tự xác định format; biết phần nào cần đổi để API và I²S hiểu đúng.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-25"></a>
## Tuần 25 — I2S DMA và bộ đệm phát

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 24. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Tính được payload/RAM/bus riêng; phát đúng tốc độ và hiểu cách khắc phục cạn buffer.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [ESP32-S3 I²S](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/peripherals/i2s.html) | Đọc Introduction, Standard Mode, clock/slot configuration và DMA buffer; phân biệt khung truyền với mẫu có ích. | Bảng sample rate, data width, slot width, channels và bit clock. |
| 2 | [Adafruit — MAX98357 I²S Amplifier](https://learn.adafruit.com/adafruit-max98357-i2s-class-d-mono-amp/) | Đọc Pinouts, Speaker Outputs, Power và wiring phù hợp; kiểm tra ngõ ra loa cầu. | Sơ đồ nguồn/tín hiệu/loa của module thật. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** vì sao CPU vẫn rảnh một lúc mà âm thanh có thể bị đứt?

**Bản chất.** I²S truyền mẫu theo nhịp clock và slot. Sample rate nói về tần suất khung/mẫu theo cấu hình; slot width nói về số bit dành trong luồng truyền. Dữ liệu hữu ích 16 hoặc 24 bit có thể nằm trong slot rộng hơn. Phải phân biệt bit clock, word select và dữ liệu.

DMA chuyển các khối giữa bộ nhớ và ngoại vi theo cấu hình, giảm việc CPU phục vụ từng mẫu. Producer phải bổ sung dữ liệu trước khi consumer phát hết. Nếu cạn buffer, âm thanh có thể gián đoạn. Buffer lớn hấp thụ dao động thời gian tốt hơn nhưng tăng bộ nhớ và có thể tăng độ trễ; nó không sửa producer chậm hơn consumer kéo dài.

MAX98357A nhận âm thanh số và khuếch đại ra loa. Loa nối theo đầu ra module; với đầu ra cầu vi sai, không tự nối một đầu loa về GND. Chọn nguồn và trở kháng theo module.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T25-LT1:** BCLK, WS/LRCLK và DATA biểu diễn gì?

2. **T25-LT2:** Tại sao mẫu 16 bit có thể đi trong slot 32 bit?

3. **T25-LT3:** DMA giảm việc gì của CPU và vẫn cần CPU làm việc gì?

4. **T25-LT4:** Buffer underrun/overrun khác nhau thế nào?

5. **T25-LT5:** Buffer lớn đánh đổi RAM/độ trễ/khả năng chịu jitter ra sao?

6. **T25-LT6:** Vì sao không nối tùy tiện một đầu loa vi sai về GND?

### E. Bài tập tính toán hoặc thiết kế

**T25-B1.** I²S 16 kHz,2 slot,32 bit/slot. Tính bit clock. So với bitrate dữ liệu mono 16 bit có ích.

**T25-B2.** PCM mono 16 kHz 16 bit; producer ngừng 150 ms. Cần bao nhiêu byte đã đệm để không cạn trong mô hình lý tưởng? Buffer 100 ms có đủ không?

### F. Bài thực hành theo trình tự

**TH1.** Lập bảng sample rate, số slot, slot width, data width và chân. Ví dụ 16 kHz, hai slot 32 bit cần bit clock 1,024 MHz theo cấu hình đó.

**TH2.** Phát âm mẫu ngắn đã biết định dạng; bắt đầu âm lượng thấp. Không gửi cả header WAV như mẫu PCM.

**TH3.** Tính 100 ms PCM mono 16 bit/16 kHz là 3200 byte dữ liệu hữu ích, phân biệt với buffer bus thực.

**TH4.** Phát 10 lần, thử thay kích thước buffer một yếu tố mỗi lần và ghi gián đoạn/độ trễ.

**Bài biến thể và kiểm tra sâu:** Phát file cố định trước khi dùng TTS. Ghi underrun và kích thước buffer, thử một thay đổi mỗi lần. Không đưa header WAV hoặc dữ liệu nén thẳng vào nơi chờ PCM.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T25-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T25-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Tính được payload/RAM/bus riêng; phát đúng tốc độ và hiểu cách khắc phục cạn buffer.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-26"></a>
## Tuần 26 — Diễn giải đúng dữ liệu từ micro

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 25. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Nghe rõ trên laptop trước STT; có khả năng truy vết từ byte thô tới mẫu và âm nghe.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [TDK — INMP441 Datasheet](https://product.tdk.com/system/files/dam/doc/product/sw_piezo/mic/mems-mic/data_sheet/inmp441.pdf) | Đọc Specifications, Pin Configurations, I²S Data Interface, Data Word Length và Data-Word Format; đối chiếu chân module. | Bảng nguồn, L/R, bit hữu ích, căn bit và timing. |
| 2 | [ESP32-S3 I²S](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/peripherals/i2s.html) | Đọc phần receive/slot/data format của driver đang dùng. | Hợp đồng từ buffer nhận tới mẫu signed và WAV. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** vì sao nhận được nhiều byte từ micro mà file vẫn toàn nhiễu?

**Bản chất.** Byte chỉ có ý nghĩa khi thống nhất kênh, số bit có ích, vị trí bit, dấu, thứ tự byte và tốc độ. INMP441 cung cấp dữ liệu I²S 24 bit; ứng dụng thường làm việc với vùng chứa rộng hơn theo cấu hình driver. Chọn sai slot/kênh có thể đọc im lặng; căn bit hoặc diễn giải số âm sai có thể tạo méo lớn.

Đổi 24 bit sang 16 bit cần xử lý có dấu, scaling và giới hạn giá trị đúng. Gắn đuôi .wav vào buffer raw không tạo header hay sửa encoding. Test bằng nghe lại trên laptop giúp tách lỗi thu khỏi lỗi STT.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T26-LT1:** Nhận byte khác 0 có chứng minh thu đúng giọng không?

2. **T26-LT2:** Chọn sai kênh/slot gây hiện tượng gì?

3. **T26-LT3:** Sai signed/unsigned hoặc căn bit làm biến dạng mẫu thế nào?

4. **T26-LT4:** Đổi 24 bit sang 16 bit cần xử lý gì?

5. **T26-LT5:** Vì sao thêm đuôi.wav không làm raw buffer thành file WAV?

6. **T26-LT6:** Làm sao tách nhiễu do âm servo, rung cơ và nguồn?

### E. Bài tập tính toán hoặc thiết kế

**T26-B1.** Xem một giá trị signed 24 bit ở dạng bù 2 có bit pattern 0xFFFFFF. Giá trị là gì? Nếu coi unsigned thì sao? Đây là ví dụ về biểu diễn, chưa nói vị trí bit trong buffer driver.

**T26-B2.** Thu 5 s ở16 kHz, lưu mỗi mẫu vào 4 byte. Tính raw bytes. Chuyển thành mono 16 bit thì payload bao nhiêu? Vì sao file thực còn có thể lớn hơn?

### F. Bài thực hành theo trình tự

**TH1.** Xác nhận module, nguồn, L/R, clock và data. Ghi rõ cách chọn slot.

**TH2.** Thu 5 s: một giây im lặng, ba giây nói, một giây im lặng. Lưu raw cùng cấu hình, không mất thông tin diễn giải.

**TH3.** Chuyển sang PCM/WAV phù hợp và nghe lại. Xem min/max và waveform; tìm cắt đỉnh, offset hoặc toàn 0.

**TH4.** Giữ câu nói và gain, thay khoảng cách 20 cm/50 cm; sau đó thử có/không servo để phân biệt nhiễu cơ/âm/điện.

**Bài biến thể và kiểm tra sâu:** Thu 1 s yên+3 s nói+1 s yên, lưu raw kèm format. Xuất file đúng, xem min/max/waveform. Giữ câu nói để thử hai khoảng cách và có/không motor.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T26-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T26-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Nghe rõ trên laptop trước STT; có khả năng truy vết từ byte thô tới mẫu và âm nghe.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-27"></a>
## Tuần 27 — Chuỗi nhận dạng hội thoại và tổng hợp tiếng nói

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 26. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Một lượt hoàn chỉnh có giới hạn, hủy và đường lỗi; không còn dùng trạng thái nói giả.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [Google Cloud — Speech-to-Text overview](https://docs.cloud.google.com/speech-to-text/docs/overview) | Đọc cấu trúc request/response, encoding, language và các chế độ nhận dạng; chọn luồng ngắn đơn giản. | Mẫu audio và transcript kiểm chứng. |
| 2 | [Google Cloud — Text-to-Speech basics](https://docs.cloud.google.com/text-to-speech/docs/basics) | Đọc text input, voice/language và audio encoding; chọn định dạng phát được. | Mẫu text, response format và bước giải mã/chuyển đổi. |
| 3 | [Gemini — Text generation](https://ai.google.dev/gemini-api/docs/text-generation) | Ôn request hội thoại đã chạy; giữ cố định trong khi kiểm tra audio. | Sơ đồ pipeline với dữ liệu tại từng điểm. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** sai ở câu trả lời cuối có thể bắt đầu từ khâu nào?

**Bản chất.** STT suy ra văn bản từ âm thanh, mô hình hội thoại sinh văn bản trả lời, TTS sinh âm thanh từ văn bản. Mỗi khâu có lỗi riêng. STT nghe sai tên rồi mô hình trả lời dựa trên tên sai không nhất thiết là mô hình tự bịa. Phải giữ các đầu ra trung gian để biết lỗi ở đâu.

Định dạng TTS có thể là file nén hoặc PCM với tốc độ khác đầu ra I²S. Cần giải mã/chuyển định dạng phù hợp, không đưa mọi byte phản hồi thẳng ra bus. Bản đầu nghe–nói lần lượt giúp tránh loa bị micro thu lại; full duplex là bài toán khác có khử tiếng vọng và ngắt lời.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T27-LT1:** STT nghe sai và LLM suy luận sai phân biệt bằng dữ liệu nào?

2. **T27-LT2:** TTS trảMP 3 có thể phát trực tiếp bằng cấu hình I²S PCM không?

3. **T27-LT3:** Vì sao half-duplex phù hợp bản đầu?

4. **T27-LT4:** Tại sao giới hạn thời gian thu và kích thước dữ liệu cần đặt trước?

5. **T27-LT5:** Mỗi chặng lỗi thì robot nên về state nào?

6. **T27-LT6:** Cần đồng bộ mắt với trạng thái nào, không chỉ thời gian giả nào?

### E. Bài tập tính toán hoặc thiết kế

**T27-B1.** Người dùng nói 'tôi tên An', STT ghi 'tôi tên Anh', AI lưu Anh. Thiết kế cách sửa để không âm thầm lưu tên sai.

**T27-B2.** Vẽ hợp đồng dữ liệu cho 5 bước:thu→STT→LLM→TTS→phát. Ghi format, giới hạn và lỗi tại từng bước.

### F. Bài thực hành theo trình tự

**TH1.** Gửi file thu tốt ở tuần 26, so transcript với câu bạn thật sự nói.

**TH2.** Gọi TTS bằng câu chữ cố định, phát đúng định dạng; tách khỏi AI.

**TH3.** Ghép cả chuỗi theo máy trạng thái. Giới hạn một lượt thu ban đầu 5 s; không thu khi phát.

**TH4.** Thử 10 câu cố định, gồm tên và sở thích. Ghi transcript, câu trả lời chữ và kết quả phát cho từng lượt.

**Bài biến thể và kiểm tra sâu:** Kiểm tra STT và TTS riêng rồi ghép 5 lượt, mở rộng 10 câu cố định. Ghi âm gốc, transcript, câu trả lời và định dạng audio để tìm đúng khâu sai.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T27-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T27-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Một lượt hoàn chỉnh có giới hạn, hủy và đường lỗi; không còn dùng trạng thái nói giả.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-28"></a>
## Tuần 28 — Đo độ trễ theo ranh giới rõ ràng

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 27. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Mọi con số có mốc/đơn vị/điều kiện; không kết luận từ cảm giác hoặc một lượt tốt.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [MDN — HTTP Overview](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Overview) | Ôn request/response để đặt mốc bắt đầu/kết thúc; phân biệt nhận byte đầu và nhận xong body. | Bảng định nghĩa các mốc đo. |
| 2 | [Google Cloud — Speech-to-Text overview](https://docs.cloud.google.com/speech-to-text/docs/overview) | Đọc phần trả kết quả của chế độ đã chọn; đối chiếu tài liệu LLM/TTS tương tự. | Xác định chặng nào nối tiếp, chặng nào có thể chồng lấp. |
| 3 | [ESP32-S3 High Resolution Timer](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/system/esp_timer.html) | Đọc Overview và cách lấy thời gian đơn điệu của nền tảng; đối chiếu clock của server. | Phân biệt đo khoảng thời gian với giờ lịch. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** con số “mất 4 giây” được tính từ lúc nào đến lúc nào?

**Bản chất.** Độ trễ chỉ có nghĩa khi chỉ rõ điểm bắt đầu/kết thúc. Thời gian người nói, upload, STT, LLM, TTS và buffer phát là các phần khác nhau. Tổng các đoạn chạy nối tiếp có thể cộng; các đoạn chồng lấp không thể cộng máy móc.

Dùng clock đơn điệu để đo khoảng thời gian trên cùng thiết bị, tránh ảnh hưởng chỉnh đồng hồ lịch. Không trừ timestamp robot và server chưa đồng bộ để kết luận one-way latency. Có thể đo end-to-end ở robot và từng chặng nội bộ ở server rồi đối chiếu.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T28-LT1:** Độ trễ tính từ nhấn nút hay từ thu xong có khác nhau không?

2. **T28-LT2:** Vì sao clock đo khoảng thời gian nên đơn điệu?

3. **T28-LT3:** Vì sao không trừ hai đồng hồ chưa đồng bộ để tính one-way latency?

4. **T28-LT4:** Trung vị và trung bình cho biết gì khác nhau khi có lượt rất chậm?

5. **T28-LT5:** Vì sao phải giữ cả lượt lỗi trong báo cáo?

6. **T28-LT6:** Tối ưu một chặng có thể không cải thiện trải nghiệm tổng vì sao?

### E. Bài tập tính toán hoặc thiết kế

**T28-B1.** Các chặng tuần tự giả định:upload 200 ms, STT 700 ms, LLM 1200 ms, TTS 500 ms, buffer 100 ms. Tính tổng. LLM giảm còn 600 ms thì tổng mới và phần trăm giảm?

**T28-B2.** 5 lượt có độ trễ 1,2; 1,4; 1,5; 1,6; 8,0 giây. Tính trung bình và trung vị. Báo một số nào có thể che mất điều gì?

### F. Bài thực hành theo trình tự

**TH1.** Định nghĩa T0 = kết thúc thu, T1 = bắt đầu phát ở robot; log khoảng T1−T0. Ở server đo riêng upload nhận xong, STT, LLM, TTS.

**TH2.** Chạy 20 lượt có request_id, cùng tập câu và điều kiện. Giữ cả lỗi, không chỉ tính những lượt đẹp.

**TH3.** Tính trung vị, nhỏ nhất, lớn nhất và số thất bại. Mẫu này chỉ mô tả bài thử, chưa chứng minh độ tin cậy lâu dài.

**TH4.** Chọn nút thắt, thay đúng một yếu tố rồi lặp. Ví dụ giới hạn câu trả lời ngắn hơn; kiểm tra chất lượng cùng độ trễ.

**Bài biến thể và kiểm tra sâu:** Chạy 20 lượt cùng tập câu, có request_id. Đo end-to-end tại robot và từng chặng trên server. Chọn một thay đổi, lặp lại và đối chiếu cả chất lượng.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T28-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T28-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Mọi con số có mốc/đơn vị/điều kiện; không kết luận từ cảm giác hoặc một lượt tốt.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-29"></a>
## Tuần 29 — Ngân sách bộ nhớ và ổn định theo thời gian

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 28. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Có vòng đời buffer và số đo; nhận biết bằng chứng leak còn chưa chắc; không chỉ mua bo nhiều RAM hơn.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [ESP32-S3 Heap Memory Debugging](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/system/heap_debug.html) | Đọc heap information, minimum free, largest free block nếu API phiên bản có và tracing khi cần. | Bảng chỉ số bộ nhớ được đo ở cùng mốc mỗi lượt. |
| 2 | [ESP32-S3 I²S](https://docs.espressif.com/projects/esp-idf/en/stable/esp32s3/api-reference/peripherals/i2s.html) | Ôn DMA/buffer và giới hạn vùng nhớ của cấu hình thực. | Sơ đồ thời gian sống chồng lấp các buffer. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** vì sao một lượt chạy được nhưng lượt thứ 50 lại thất bại?

**Bản chất.** Một lượt có thể đồng thời giữ audio raw, file chuyển đổi, buffer mạng/TLS, JSON và ảnh màn hình. Cộng dung lượng payload đơn lẻ không đủ. Tổng RAM còn trống cũng chưa chắc cho phép cấp một khối liên tục đủ lớn nếu bị phân mảnh.

Một số tài nguyên được giải phóng sau lượt; một số được cache hợp lệ. Cần nhìn xu hướng sau nhiều lượt ở cùng mốc, không coi mọi tăng sử dụng ban đầu là leak. Queue không được xử lý kịp hoặc request chưa đóng có thể làm sử dụng tăng dần.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T29-LT1:** Peak memory khác tổng dung lượng từng file xử lý ra sao?

2. **T29-LT2:** Tại sao tổng RAM trống đủ vẫn có thể không cấp được một block?

3. **T29-LT3:** Warm-up/cache tăng ban đầu khác leak tăng mãi thế nào?

4. **T29-LT4:** Queue tồn đọng có thể giả dạng leak ra sao?

5. **T29-LT5:** Rust an toàn bộ nhớ có ngăn mọi kiểu cạn RAM không?

6. **T29-LT6:** Muốn khoanh vùng leak nên tắt/cố định phần nào và vì sao?

### E. Bài tập tính toán hoặc thiết kế

**T29-B1.** Cùng lúc giữ audio raw 320 kB, PCM 160 kB, network 64 kB, output 160 kB và phần khác 100 kB. Tính tổng theo kB thập phân. Nếu có thể giải phóng raw trước khi tạo output thì đỉnh hai pha là bao nhiêu?

**T29-B2.** Sau mỗi lượt heap mất đều 2 KiB chưa hồi; ngân sách còn 200 KiB. Sau bao nhiêu lượt sẽ mất thêm 100 KiB? Vì sao không dùng con số này để hứa chạy được đủ 100 lượt?

### F. Bài thực hành theo trình tự

**TH1.** Lập bảng buffer, kích thước, nơi cấp phát, nơi giải phóng và thời điểm sống chồng lên nhau.

**TH2.** Log heap tại cùng mốc trước/sau mỗi lượt; ghi cả queue length, lỗi audio và reset reason khi có.

**TH3.** Chạy một giờ với lượt định kỳ. So mức sau khởi động ấm với về cuối.

**TH4.** Nếu có xu hướng xấu, tắt một chức năng hoặc thay bằng phản hồi giả rồi lặp để khoanh vùng. Sửa và đo lại.

**Bài biến thể và kiểm tra sâu:** Chạy 1 giờ với mốc log cố định, vẽ bảng heap/queue/lỗi. Nếu giảm đều, dùng phản hồi giả thay một dịch vụ rồi lặp để tách nguyên nhân.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T29-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T29-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Có vòng đời buffer và số đo; nhận biết bằng chứng leak còn chưa chắc; không chỉ mua bo nhiều RAM hơn.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-30"></a>
## Tuần 30 — Kiểm thử giả thuyết và khả năng phục hồi

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 29. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Người khác tái hiện được lỗi từ bản ghi; các ca ảnh hưởng cốt lõi được sửa và thử lại.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [MDN — HTTP status codes](https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Status) | Ôn mã lỗi liên quan ứng dụng; phân biệt lỗi client/server/dịch vụ. | Danh sách phản ứng theo từng loại lỗi. |
| 2 | [MDN — HTTP Overview](https://developer.mozilla.org/en-US/docs/Web/HTTP/Guides/Overview) | Ôn cấu trúc giao tiếp để tạo lỗi giả có kiểm soát. | Bộ kịch bản tái hiện độc lập model. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** làm sao chứng minh một bản sửa đã xử lý nguyên nhân chứ không chỉ tình cờ hết lỗi?

**Bản chất.** Một lỗi có thể quan sát ở đầu ra nhưng bắt nguồn từ lớp khác. Cần điều kiện tái hiện, kết quả mong muốn, kết quả thật và log nhận diện lượt. Thay nhiều thứ cùng lúc làm mất khả năng quy nguyên nhân.

Happy path kiểm tra luồng thuận lợi; fault injection tạo lỗi có kiểm soát ở phần mềm/mạng để xem phục hồi. Không dùng chập nguồn hoặc kẹt motor làm cách tạo lỗi. Sau sửa cần thử lại ca đã lỗi và các ca dễ bị ảnh hưởng; không cần lặp mọi kiểm thử vô hạn khi đã đủ bằng chứng cho phạm vi thay đổi.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T30-LT1:** Một lần demo thành công chứng minh được phạm vi nào?

2. **T30-LT2:** Hypothesis và observation khác nhau ra sao trong báo cáo lỗi?

3. **T30-LT3:** Vì sao sửa nhiều yếu tố cùng lúc làm khó kết luận?

4. **T30-LT4:** Happy path, boundary case và fault injection khác nhau thế nào?

5. **T30-LT5:** Regression test nên chọn theo ảnh hưởng bản sửa như thế nào?

6. **T30-LT6:** Vì sao không tạo lỗi điện bằng chập nguồn/giữ motor kẹt để học?

### E. Bài tập tính toán hoặc thiết kế

**T30-B1.** Một bộ 20 ca chạy được 18, hai ca lỗi cùng nguyên nhân mạng. Tính tỷ lệ qua bộca và nêu điều không thể suy ra về độ tin cậy ngoài thực tế.

**T30-B2.** Robot reset lúc servo chạy. Viết hai giả thuyết khác nhau và hai thử nghiệm có thể phân biệt mà không gây hại.

### F. Bài thực hành theo trình tự

**TH1.** Viết thao tác tái hiện cho từng ca và điều kiện đạt.

**TH2.** Chạy có request_id và lưu log trước khi sửa.

**TH3.** Chọn lỗi ảnh hưởng nhất, đề xuất hai giả thuyết và phép thử phân biệt.

**TH4.** Sửa dựa trên bằng chứng, lặp ca đó và một ca lân cận.

**Bài biến thể và kiểm tra sâu:** Tạo 12 ca gồm mất mạng, timeout, trùng, hủy, JSON sai, body quá lớn, âm thanh sai format, thiếu hồ sơ, đổi tên, quên, restart và tải đồng thời. Mỗi ca có thao tác, mong đợi, kết quả, log và cách phục hồi.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T30-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T30-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Người khác tái hiện được lỗi từ bản ghi; các ca ảnh hưởng cốt lõi được sửa và thử lại.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-31"></a>
## Tuần 31 — Vỏ là một phần của hệ thống

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 30. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Vỏ không che âm/khóa chuyển động/kéo dây; có cách tháo để sửa và số đo so trước/sau.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [TDK — INMP441 Datasheet](https://product.tdk.com/system/files/dam/doc/product/sw_piezo/mic/mems-mic/data_sheet/inmp441.pdf) | Đọc kích thước, vị trí cổng âm và khuyến nghị lắp liên quan micro thực; phân biệt chip rời với module. | Bản vẽ hướng cổng âm so với vỏ. |
| 2 | [OpenStax — Torque and Equilibrium](https://openstax.org/books/college-physics-2e/pages/9-2-the-second-condition-for-equilibrium) | Ôn trọng tâm/mô-men khi bố trí phần đầu và chân đế. | Sơ đồ lực và khoảng chuyển động. |
| 3 | [Adafruit — MAX98357 I²S Amplifier](https://learn.adafruit.com/adafruit-max98357-i2s-class-d-mono-amp/) | Ôn vị trí loa, dây nguồn và đầu nối theo module. | Bố trí đường âm, nguồn và tháo sửa. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** vì sao mạch chạy tốt trên bàn lại nghe kém hoặc rung khi đặt vào vỏ?

**Bản chất.** Vỏ thay đổi đường âm, cộng hưởng, rung, lực dây, khoảng chuyển động và khả năng tản nhiệt. Cổng âm của micro cần tiếp xúc với đường âm phù hợp; không che chỉ vì module vẫn nhận điện. Loa có thể truyền rung qua vỏ tới micro; servo có thể tạo âm và kéo dây.

Trọng tâm và diện tích chân đế quyết định khả năng đổ theo lực tác dụng. Kích thước phải chừa dung sai lắp, dây/đầu nối và đường tháo sửa; đúng kích thước linh kiện trên giấy chưa đủ.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T31-LT1:** Vỏ có thể đổi âm thanh bằng những cơ chế nào?

2. **T31-LT2:** Vì sao không bịt cổng âm dù micro vẫn nhận điện?

3. **T31-LT3:** Dây có thể trở thành tải cơ khí thế nào?

4. **T31-LT4:** Dung sai lắp và khoảng trống cho đầu nối cần xét ra sao?

5. **T31-LT5:** Trọng tâm và chân đế ảnh hưởng khả năng đổ thế nào?

6. **T31-LT6:** Vì sao cần thử lại sau mỗi thay đổi bố trí thay vì chỉ kiểm tra trước lắp vỏ?

### E. Bài tập tính toán hoặc thiết kế

**T31-B1.** Hai linh kiện rộng 30 mm và 25 mm cần khoảng cách 5 mm, khoảng hở hai mép mỗi bên 3 mm. Tính chiều rộng khoang tối thiểu theo mô hình bố trí thẳng.

**T31-B2.** Thiết kế thử nghiệm so âm thanh trước/sau vỏ sao cho chỉ thay yếu tố vỏ. Liệt kê tối thiểu 6 điều kiện giữ cố định.

### F. Bài thực hành theo trình tự

**TH1.** Đo linh kiện thật, vẽ mặt trước/bên, đánh dấu trục, cổng âm, loa, nút và nguồn.

**TH2.** Làm vỏ bìa nhẹ, đặt linh kiện tạm có thể di chuyển.

**TH3.** Dùng cùng câu nói, khoảng cách và âm lượng để so trước/sau vỏ. Thử quay đầu quan sát dây và độ vững.

**TH4.** Sửa một vị trí gây ảnh hưởng rồi đo lại; chốt kích thước sau khi bài thử đạt. CAD/in 3D chỉ cần khi muốn vỏ chính xác hơn.

**Bài biến thể và kiểm tra sâu:** Làm vỏ bìa trước, chụp ba hướng kèm kích thước. So thu/phát, rung, lực dây và ổn định. Chỉ chốt in 3D sau khi bố trí đạt.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T31-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T31-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Vỏ không che âm/khóa chuyển động/kéo dây; có cách tháo để sửa và số đo so trước/sau.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-32"></a>
## Tuần 32 — Kết nối điện cũng là kết cấu cơ khí

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 31. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Bảng dây khớp bản thật; không có lỗi mới sau lắp; kết nối chịu chuyển động thông thường trong phạm vi thiết kế.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [Adafruit — Guide to Excellent Soldering](https://learn.adafruit.com/adafruit-guide-excellent-soldering) | Đọc Tools, Making a Joint và Common Problems; xem hình mối hàn đạt/lỗi. | Checklist kiểm tra mối nối điện và cơ. |
| 2 | [Adafruit — Multimeters](https://learn.adafruit.com/multimeters) | Ôn continuity/resistance đúng cách trước cấp nguồn. | Quy trình kiểm tra rail và đầu nối. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** vì sao đo thông mạch được nhưng thiết bị vẫn lỗi lúc rung?

**Bản chất.** Tiếp xúc có thể thay đổi theo lực, oxy hóa, nhiệt và dòng. Bài continuity dùng điều kiện nhỏ không chứng minh kết nối chịu được tải động lâu dài. Dây cần tiết diện/phân phối phù hợp, mối nối cần giữ chắc và cơ cấu chống kéo để lực không dồn vào pad.

Mối hàn cần làm ướt bề mặt và liên kết đúng; một cục thiếc lớn không tự đồng nghĩa mối nối tốt. Kiểm tra điện và kiểm tra cơ bổ sung cho nhau. Breadboard rất tốt để học, nhưng dây cắm lỏng không thích hợp mọi thiết bị sử dụng thường xuyên.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T32-LT1:** Continuity ở dòng nhỏ khác kiểm tra chịu tải thế nào?

2. **T32-LT2:** Lực kéo truyền vào pad và mối hàn có thể gây lỗi gì?

3. **T32-LT3:** Mối hàn lớn có chắc là mối hàn tốt không?

4. **T32-LT4:** Nhãn, màu dây và phân cực đầu nối giúp giảm loại lỗi nào?

5. **T32-LT5:** Vì sao phải đổi từng nhóm dây và thử lại?

6. **T32-LT6:** Tại sao đo giữa rail có thể thay đổi theo thời gian khi có tụ?

### E. Bài tập tính toán hoặc thiết kế

**T32-B1.** Đầu nối có điện trở tiếp xúc giả định 0,2 Ω, dòng 1 A. Tính sụt áp và công suất nhiệt. Nếu dòng 2 A, hai đại lượng đổi thế nào?

**T32-B2.** Lập ma trận kiểm tra nối dây gồm net, điểm A/B, đo thông mạch mong đợi, không được nối với net nào và kết quả. Nêu ít nhất 8 đường của robot.

### F. Bài thực hành theo trình tự

**TH1.** Lập bảng đầu nối, số chân, nhãn và màu dây; tập trên bo thử nếu hàn.

**TH2.** Chuyển từng nhóm dây, kiểm tra với sơ đồ sau mỗi nhóm; không đổi toàn bộ rồi đoán lỗi.

**TH3.** Ngắt nguồn, kiểm tra chập giữa rail và continuity theo điểm. Kiểm tra cực tụ và đầu nối.

**TH4.** Cấp nguồn, chạy từng chức năng rồi kết hợp; di chuyển bình thường trong phạm vi cơ cấu để kiểm tra lực dây.

**Bài biến thể và kiểm tra sâu:** Luyện hàn trên bo rời nếu tự hàn, hoặc giao sơ đồ rõ nếu thuê. Đổi từng nhóm dây; kiểm tra khi ngắt nguồn rồi thử chức năng. Cố định dây tránh lực lên pad.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T32-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T32-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Bảng dây khớp bản thật; không có lỗi mới sau lắp; kết nối chịu chuyển động thông thường trong phạm vi thiết kế.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="tuan-33"></a>
## Tuần 33 — Tái lập sao lưu và ranh giới bản đầu

### A. Tiên quyết và mục tiêu

Hoàn thành đầu ra tuần 32. Nếu kiến thức của tuần trước còn vướng, làm lại bài liên quan trước khi ghép thêm chức năng.

**Mục tiêu cuối tuần:** Người khác có đủ dữ kiện để tái lập; backup được phục hồi; hoàn thành bộ kiểm tra và nêu giới hạn.

### B. Tài liệu bắt buộc và phần cần đọc

Đọc theo thứ tự từ trên xuống. Sau mỗi nguồn, hoàn thành cột “Việc phải làm sau đọc” trước khi đọc tiếp.

| Thứ tự | Tài liệu | Đọc phần nào | Việc phải làm sau đọc |
|---|---|---|---|
| 1 | [SQLite — Command Line Shell](https://www.sqlite.org/cli.html) | Tìm .backup và cách restore/mở bản sao; đọc lưu ý phù hợp chế độ database đang chạy. | Bản sao lưu đã được khôi phục thử. |
| 2 | [Caddy — Reverse proxy quick start](https://caddyserver.com/docs/quick-starts/reverse-proxy) | Tùy chọn khi đưa server khỏi laptop: đọc reverse proxy và ví dụ; học HTTPS/xác thực theo môi trường thật. | Sơ đồ triển khai, nơi giữ khóa, cấu hình và backup. |

### C. Định hướng để đọc hiểu bản chất

**Câu hỏi trung tâm:** sau một tháng, có thể dựng lại hệ thống mà không dựa vào trí nhớ không?

**Bản chất.** Mã nguồn là một phần của hệ chạy được. Còn phiên bản dependency, toolchain, cấu hình, firmware, sơ đồ chân, dữ liệu và quy trình khởi động. Bí mật phải được tách khỏi cấu hình mẫu và tài liệu chia sẻ. Sao lưu chỉ có giá trị khi khôi phục được trong bài thử.

Firmware và server cần cùng hợp đồng giao tiếp. Chuyển server sang máy khác là thay môi trường vận hành, không làm dịch vụ AI trở thành offline. Triển khai ngoài LAN cần kiểm tra HTTPS, xác thực, giới hạn yêu cầu, vận hành và dữ liệu; không cần ép vào cùng ngày hoàn thiện nếu bản laptop chưa ổn.

### D. Bài tập lý thuyết phải tự trả lời

Viết mỗi câu thành một đoạn có lập luận hoặc sơ đồ. Nếu dùng công thức, nêu giả thiết và đơn vị. Không chỉ liệt kê từ khóa.

1. **T33-LT1:** Có mã nguồn vì sao chưa chắc tái lập được bản chạy?

2. **T33-LT2:** Lockfile, toolchain, firmware version và protocol version khác nhau thế nào?

3. **T33-LT3:** Cấu hình mẫu khác khóa bí mật thật thế nào?

4. **T33-LT4:** Sao lưu thành công khác khôi phục thành công thế nào?

5. **T33-LT5:** Chuyển server lên cloud có làm AI offline không?

6. **T33-LT6:** Những giới hạn nào phải ghi trong bản đầu để người dùng hiểu đúng?

### E. Bài tập tính toán hoặc thiết kế

**T33-B1.** Viết kế hoạch tái lập trên một thư mục sạch: liệt kê input cần có, lệnh build/nạp, cấu hình và kiểm tra sau mỗi bước. Không giả định người đọc nhớ chat cũ.

**T33-B2.** Sao lưu hồ sơ A, sửa/xóa trên bản thử, phục hồi từ backup. Thiết kế cách so sánh để chứng minh đúng người và đúng thông tin.

### F. Bài thực hành theo trình tự

**TH1.** Tạo bản phát hành gồm mã/lockfile, phiên bản, sơ đồ, BOM, cấu hình mẫu và hướng dẫn khởi động; không đưa khóa thật.

**TH2.** Sao lưu dữ liệu bằng cách phù hợp SQLite, phục hồi vào bản thử riêng và đọc lại hồ sơ.

**TH3.** Từ trạng thái tắt, làm theo tài liệu của chính mình để chạy toàn hệ. Ghi bước còn phải “tự nhớ” rồi bổ sung.

**TH4.** Chạy bộ nghiệm thu cố định, ghi giới hạn thật về khoảng cách nói, độ trễ, tải servo, mạng và thời lượng chạy đã thử.

**Bài biến thể và kiểm tra sâu:** Tạo release có BOM, sơ đồ nguồn/dây, phần mềm, hướng dẫn và báo cáo. Tắt toàn bộ rồi tự chạy từ tài liệu; bổ sung mọi bước còn phải nhớ. Nếu chưa triển khai cloud, bản laptop hoàn chỉnh là đầu ra hợp lệ.

### G. Hồ sơ cần hoàn thành và điều kiện chuyển tuần

- [ ] Ghi chú đọc tài liệu: định nghĩa bằng lời riêng, sơ đồ, công thức/đơn vị và điều chưa chắc.
- [ ] Trả lời đủ sáu câu T33-LT; kiểm lại bằng tài liệu ở phần B.
- [ ] Giải hai bài T33-B trước khi mở đáp án; nếu sai, ghi sai ở giả thiết, phép tính hay lập luận.
- [ ] Hoàn thành TH1–TH4; ghi dữ liệu gốc, ảnh/sơ đồ hoặc log thích hợp.
- [ ] Làm bài biến thể, có dự đoán trước và kết quả sau.
- [ ] **Đạt:** Người khác có đủ dữ kiện để tái lập; backup được phục hồi; hoàn thành bộ kiểm tra và nêu giới hạn.

Nếu chưa đạt, quay lại nguồn ứng với điểm sai; không coi việc sao chép một đoạn code khác làm bằng chứng hiểu.


<a id="nghiem-thu"></a>
## 6. Các mốc nghiệm thu

Bạn nên dừng ở mỗi mốc để kiểm tra toàn nhóm kiến thức. Không mua/thêm chức năng chỉ để né lỗi chưa hiểu.

| Mốc | Bài nghiệm thu | Bằng chứng tối thiểu |
|---|---|---|
| Sau tuần 6 | Tự vẽ/lắp LED–nút, giải chia áp có tải, đo RC và đọc định mức linh kiện | Sơ đồ, phép tính, bảng đo, giải thích một sai lệch |
| Sau tuần 12 | Nút đếm đúng, ADC có so với đồng hồ, PWM thay được, OLED biểu cảm không chặn nút | Log, bảng đo, 3 biểu cảm, giải thích RAM/bus |
| Sau tuần 17 | Servo có giới hạn, nguồn ổn trong ca thử, hủy khi đang chờ được | Sơ đồ nguồn, bảng trạng thái, log chậm/đầy/hủy |
| Sau tuần 20 | Giao tiếp server có hợp đồng và lỗi có kiểm soát | 10 payload thử, timeline timeout, ca yêu cầu trùng |
| Sau tuần 23 | Hội thoại chữ có hồ sơ, sửa và quên đúng người | Database thử, context trước/sau, bộ ca A/B |
| Sau tuần 27 | Thu rõ, phát đúng, hỏi đáp giọng nói hoàn chỉnh | Raw/format/WAV, transcript, text trả lời, 10 ca audio |
| Sau tuần 30 | Đo được độ trễ/bộ nhớ, tìm và sửa ít nhất một nguyên nhân lỗi | Bảng đo, log, giả thuyết và so sánh trước/sau |
| Sau tuần 33 | Có vỏ/kết nối, tài liệu tái lập, backup đã khôi phục | BOM, schematic, phiên bản, hướng dẫn, báo cáo giới hạn |

### Tự kiểm tra cuối lộ trình

Chọn ngẫu nhiên một thay đổi: thay màn hình, đổi servo, tăng thời gian thu từ 5 lên 10 giây, chuyển server sang máy khác hoặc đổi dịch vụ STT. Trước khi thực hiện, viết:

1. Những giả thiết cũ nào có thể không còn đúng?
2. Cần đọc lại datasheet/tài liệu nào?
3. Những phần nào phải đổi và những phần nào giữ nguyên?
4. Công suất/RAM/băng thông/độ trễ có thể đổi thế nào?
5. Phép thử nhỏ nào xác nhận tương thích trước khi tích hợp?
6. Các ca regression nào cần chạy lại?

Bạn không cần thực hiện cả năm thay đổi; cần chứng minh biết cách đánh giá một thay đổi có hệ thống.

<a id="mau"></a>
## 7. Mẫu ghi chép và báo cáo thực nghiệm

### 7.1. Mẫu ghi chú đọc một bài

```markdown
# Tên bài và tuần

- Link và phần đã đọc:
- Ngày, phiên bản tài liệu nếu có:
- Câu hỏi mình muốn trả lời:
- Ba đến năm ý chính bằng lời của mình:
- Sơ đồ hoặc mô hình:
- Công thức và ý nghĩa từng ký hiệu/đơn vị:
- Điều kiện để mô hình đúng:
- Một ví dụ của robot:
- Một phản ví dụ hoặc chỗ dễ nhầm:
- Câu mình vẫn chưa giải thích được:
- Trang/mục sẽ đọc lại:
```

### 7.2. Mẫu làm bài tính

```markdown
## Txx-Bx

1. Dữ kiện và điều cần tìm:
2. Đổi đơn vị:
3. Sơ đồ/mô hình:
4. Giả thiết:
5. Suy công thức hoặc lựa chọn phương pháp:
6. Thay số/thiết kế:
7. Kết quả có đơn vị:
8. Kiểm tra hợp lý: tăng/giảm một dữ kiện có tạo xu hướng đúng không?
9. Đối chiếu đáp án sau khi tự làm:
10. Nếu sai: sai ở đâu và bài biến thể nào chứng minh đã sửa cách hiểu?
```

### 7.3. Mẫu báo cáo thực hành

```markdown
## Tuần / bài / ngày

- Mục tiêu:
- Bo/module, mã linh kiện, điện áp và phiên bản phần mềm:
- Sơ đồ dây hoặc luồng dữ liệu:
- Điều kiện giữ cố định:
- Biến duy nhất thay đổi:
- Dự đoán trước khi chạy:
- Phép tính hỗ trợ dự đoán:
- Điểm đo và công cụ:
- Kết quả thô:
- Sai lệch so với dự đoán:
- Hai giả thuyết giải thích:
- Phép thử giúp phân biệt hai giả thuyết:
- Kết luận được bằng chứng hỗ trợ:
- Điều chưa đủ bằng chứng:
- Ảnh/log/file đo/commit:
- Tiêu chí đã đạt và còn thiếu:
```

### 7.4. Mẫu bảng số đo điện

| Cấu hình | Điện áp nguồn dưới tải | Điện áp trên R | R đo thực | I = VR/R | Dự đoán | Giải thích sai lệch |
|---|---:|---:|---:|---:|---:|---|
| Điền sau khi tự làm | | | | | | |

### 7.5. Mẫu bảng kiểm thử hệ thống

| Mã ca | Điều kiện ban đầu | Tác động | Mong đợi | Kết quả thật | Đạt/chưa đạt | Log/bằng chứng |
|---|---|---|---|---|---|---|
| TC01 | | | | | | |

### 7.6. Khi mắc kẹt

- **Không hiểu khái niệm:** đọc lại nguồn nền tảng, tự vẽ ví dụ đơn giản hơn; chưa ghép robot.
- **Không ra kết quả bài tính:** kiểm tra đơn vị, nối tiếp/song song, giả thiết và miền áp dụng công thức.
- **Mạch không chạy:** kiểm tra cơ khí → nguồn → dây/chân → tín hiệu → driver → logic → mạng/dịch vụ.
- **Code Rust không build:** tách log toolchain/dependency; không thay dây ngẫu nhiên để sửa lỗi biên dịch.
- **Chạy riêng được, ghép lỗi:** kiểm tra tài nguyên dùng chung, nguồn đỉnh, thời gian, bộ đệm và quyền sở hữu.
- **Không biết hỏi gì:** gửi sơ đồ, phiên bản, điều mong đợi, điều quan sát, log và các phép thử đã làm; một ảnh “không chạy” thường thiếu dữ kiện.

<a id="dap-an"></a>
## 8. Đáp án và hướng kiểm tra 66 bài tập

**Chỉ đọc sau khi tự làm.** Các đáp án số dùng giả thiết trong đề, không tự thay thế datasheet hoặc số đo thực tế. Bài thiết kế có thể có nhiều phương án đúng; đáp án nêu những yếu tố cần có. Sáu câu lý thuyết mỗi tuần phải đối chiếu lập luận trong nguồn đã giao, không chấm bằng việc dùng đúng từ khóa.

### Tuần 01

**T01-B1 — Đối chiếu:** I = 3 mA; Q = 0,06 C; P = 9 mW; E = 0,18 J.

**T01-B2 — Đối chiếu:** 6 V: 6 mA, 36 mW. 3 V/2 kΩ: 1,5 mA, 4,5 mW. Gấp đôi V làm P gấp bốn; gấp đôi R khi V cố định làm P còn nửa.

### Tuần 02

**T02-B1 — Đối chiếu:** Khoảng 2,727 V do chia áp 1 MΩ và 10 MΩ. Vôn kế lý tưởng trở kháng vô hạn sẽ đọc 3 V tại đầu ra không tải.

**T02-B2 — Đối chiếu:** 950–1050 Ω. 980 Ω nằm trong khoảng; chưa đủ để chứng minh độ chính xác đồng hồ.

### Tuần 03

**T03-B1 — Đối chiếu:** R lý tưởng 325 Ω; với 330 Ω: I ≈ 3,94 mA; PR ≈ 5,12 mW. Đây là mô hình Vf cố định.

**T03-B2 — Đối chiếu:** Lần lượt khoảng 4,55 mA và 3,33 mA. Vf thay đổi làm điện áp trên R thay đổi.

### Tuần 04

**T04-B1 — Đối chiếu:** I = 2 mA; V1 = 2 V, V2 = 4 V; P1 = 4 mW, P2 = 8 mW.

**T04-B2 — Đối chiếu:** I1 = 6 mA, I2 = 3 mA, Itổng = 9 mA; Req ≈ 666,7 Ω; P = 54 mW.

### Tuần 05

**T05-B1 — Đối chiếu:** Không tải 1,5 V; có tải R2||RL = 500 Ω nên Vout = 1 V.

**T05-B2 — Đối chiếu:** 10 kΩ: 0,33 mA và 1,089 mW. 1 kΩ: 3,3 mA và 10,89 mW; bỏ qua dòng input.

### Tuần 06

**T06-B1 — Đối chiếu:** τ = 10 s; Vc ≈ 1,896 V; 2,851 V; 2,980 V theo Vc = 3(1−e^(−t/10)).

**T06-B2 — Đối chiếu:** ΔV = IΔt/C = 1 V; cần 1000 µF theo mô hình lý tưởng. Chưa tính ESR/ESL và nguồn đồng thời.

### Tuần 07

**T07-B1 — Đối chiếu:** Không. Cần partition table, bootloader/cấu hình, vùng NVS và các slot OTA nếu có; kích thước vật lý không bằng dung lượng một partition.

**T07-B2 — Đối chiếu:** 64.000 byte = 62,5 KiB. Chưa, còn điều kiện vùng nhớ/căn chỉnh/khả năng driver và phiên bản.

### Tuần 08

**T08-B1 — Đối chiếu:** Khoảng 1,3 mA; với 3,2 V là 1,2 mA. Phải đối chiếu giới hạn điện thực và Vf.

**T08-B2 — Đối chiếu:** LOW, vùng không bảo đảm, HIGH. Đây là thông số giả định để luyện đọc; phải dùng datasheet chip thật.

### Tuần 09

**T09-B1 — Đối chiếu:** 39 ms; lần đổi cuối về LOW ở 19 ms. Với polling rời rạc, thời điểm chấp nhận có thể muộn hơn.

**T09-B2 — Đối chiếu:** Không có mẫu nào nằm trong xung. Cần chu kỳ lấy mẫu phù hợp hoặc bắt sự kiện phần cứng; vẫn phải xử lý dội.

### Tuần 10

**T10-B1 — Đối chiếu:** Bước ≈0,806 mV; mã 2048 ≈1,6504 V. Cần dải/attenuation, calibration và đặc tính ADC thực.

**T10-B2 — Đối chiếu:** Chu kỳ 1 ms; bật 0,25/0,5/0,75 ms. 2 kHz: chu kỳ 0,5 ms, bật 0,125 ms.

### Tuần 11

**T11-B1 — Đối chiếu:** Byte ghi 0x78, byte đọc 0x79. API yêu cầu địa chỉ 7 bit nhận 0x3C; không tự dịch hai lần.

**T11-B2 — Đối chiếu:** 17×9/100000 = 1,53 ms. Thời gian thực còn overhead, chờ và cấu hình thiết bị.

### Tuần 12

**T12-B1 — Đối chiếu:** 1 khung 1024 byte = 1 KiB; 3 khung 3072 byte = 3 KiB, chưa tính cấu trúc phụ.

**T12-B2 — Đối chiếu:** 92,16 ms; khoảng 10,85 khung/s. Thực tế thấp hơn do địa chỉ/lệnh/chờ và hoạt động khác.

### Tuần 13

**T13-B1 — Đối chiếu:** 5%, 7,5%, 10%. Cần đối chiếu datasheet trước dùng phần cứng.

**T13-B2 — Đối chiếu:** Từ 1,5 ms xuống 0,75 ms. Không, servo thường diễn giải độ rộng xung; có thể ra ngoài dải hỗ trợ.

### Tuần 14

**T14-B1 — Đối chiếu:** 0,014715 N·m; ở 6 cm là 0,02943 N·m. Chưa gồm ma sát, quán tính và dự phòng.

**T14-B2 — Đối chiếu:** Quán tính gấp bốn; thành phần mô-men Iα gấp bốn. Đây là mô hình để thấy xu hướng, không phải mọi đầu robot là một điểm.

### Tuần 15

**T15-B1 — Đối chiếu:** ΔV = 0,32 V; P = 0,256 W; tải khoảng 4,68 V.

**T15-B2 — Đối chiếu:** Tổng 1,48 A; nhân 1,3 là 1,924 A. Phải xét rail điện áp, hiệu suất nếu chuyển đổi, đáp ứng quá độ, dây, tải đo thực và định mức nhà sản xuất.

### Tuần 16

**T16-B1 — Đối chiếu:** 8 tổ hợp; 4 hợp lệ. Các tổ hợp còn lại cần bị loại bởi thiết kế.

**T16-B2 — Đối chiếu:** Idle→Listening→Thinking→Idle; Result(old) bị bỏ; Start mới→Listening. Cancel phải vô hiệu phiên và dọn tài nguyên theo thiết kế.

### Tuần 17

**T17-B1 — Đối chiếu:** Khoảng 2 s vì tích lũy 20 sự kiện/s; burst có thể làm đầy sớm hơn.

**T17-B2 — Đối chiếu:** Không. Giảm vùng giữ khóa, tách tài nguyên, dùng message passing/owner task hoặc cơ chế phù hợp; phải đo lại thời gian phản hồi.

### Tuần 18

**T18-B1 — Đối chiếu:** Loopback trỏ về robot. URL dự kiến http://192.168.1.20:8080/health nếu server lắng nghe LAN và mạng cho phép.

**T18-B2 — Đối chiếu:** Không có response thì chưa có status server để kết luận. Kiểm tra DNS/IP, kết nối TCP, TLS nếu dùng, request rồi app.

### Tuần 19

**T19-B1 — Đối chiếu:** Ba trường hợp đầu không đúng hợp đồng; trường hợp cuối hợp lệ về schema nhưng vẫn phải kiểm tra phiên, quyền và trạng thái ứng dụng.

**T19-B2 — Đối chiếu:** Biên lần đọc không phải biên thông điệp; dùng framing do HTTP cung cấp hoặc quy tắc độ dài/delimiter có xử lý đúng.

### Tuần 20

**T20-B1 — Đối chiếu:** Lệnh có thể đã thực hiện trước timeout. Cần mã thao tác và xử lý trùng phù hợp; server/firmware kiểm tra phiên, không chỉ gửi lại vô điều kiện.

**T20-B2 — Đối chiếu:** 5+1+5+2+5 =18 s. Thực tế phụ thuộc định nghĩa timeout từng tầng; phải đặt deadline tổng nếu cần cứng.

### Tuần 21

**T21-B1 — Đối chiếu:** Bộ thử nên có hỏi tên, tham chiếu lượt trước, hỏi ngoài dữ liệu, đổi xưng hô và câu mới; chấm theo thông tin thực gửi, không coi mọi nhớ đúng là bộ nhớ tự có.

**T21-B2 — Đối chiếu:** Trên bộ thử này tỷ lệ đạt 70%/90%; chưa đủ kết luận tổng quát. Cần xem tiêu chí độ dài, chi phí, độ trễ và tính lặp lại.

### Tuần 22

**T22-B1 — Đối chiếu:** Cần user_id ổn định, khóa hàng sở thích và WHERE đúng; kiểm tra số hàng bị thay đổi. Nêu cách cấu hình/ràng buộc quan hệ theo SQLite thực dùng.

**T22-B2 — Đối chiếu:** Không transaction có thể mất sở thích mà chưa thêm mới. Transaction commit cả hai hoặc rollback theo xử lý lỗi.

### Tuần 23

**T23-B1 — Đối chiếu:** Hồ sơ xác nhận mới phải ưu tiên; cập nhật/vô hiệu tóm tắt dẫn xuất liên quan, không tiếp tục đưa trà như dữ kiện hiện hành.

**T23-B2 — Đối chiếu:** Đáp án là bộ ca kiểm được cả dữ liệu và context. Chỉ nghe AI nói 'đã quên' không đủ; A/B không lẫn, không bịa thông tin chưa có.

### Tuần 24

**T24-B1 — Đối chiếu:** 160.000; 320.000; 320.000; 320.000 byte tương ứng. Chưa header/buffer khác.

**T24-B2 — Đối chiếu:** 10 s và 20 s; tốc độ/cao độ giảm một nửa khi không dùng thuật toán giữ cao độ.

### Tuần 25

**T25-B1 — Đối chiếu:** BCLK = 1.024.000 bit/s; payload mono 16 bit=256.000 bit/s. Bus có thể truyền nhiều bit hơn payload có ích.

**T25-B2 — Đối chiếu:** 4800 byte; 100 ms chỉ 3200 byte nên không đủ. Cần xét biên dự phòng, slot/DMA và điều kiện thật.

### Tuần 26

**T26-B1 — Đối chiếu:** Signed 24 bit là −1; unsigned là 16.777.215. Phải xác định căn bit trước khi áp chuyển đổi.

**T26-B2 — Đối chiếu:** 320.000 byte raw; 160.000 byte PCM16; có header/metadata, padding hoặc format khác theo cách lưu.

### Tuần 27

**T27-B1 — Đối chiếu:** Giữ transcript để chẩn đoán; yêu cầu xác nhận trường quan trọng trước lưu; cho sửa. Không chỉ đổi prompt rồi bỏ qua dữ liệu STT.

**T27-B2 — Đối chiếu:** Đáp án cần có raw/PCM/file đúng metadata; transcript UTF-8; input/context cho LLM; audio encoding của TTS; PCM/slot cho phát; giới hạn và đường lỗi rõ.

### Tuần 28

**T28-B1 — Đối chiếu:** 2700 ms; 2100 ms; giảm 600/2700≈22,22%. Chỉ cộng như vậy khi các chặng thực sự nối tiếp và định nghĩa không chồng.

**T28-B2 — Đối chiếu:** Trung bình 2,74 s; trung vị 1,5 s. Chỉ trung bình che lượt thường nhanh, chỉ trung vị che đuôi 8 s; cần thêm phạm vi và lỗi.

### Tuần 29

**T29-B1 — Đối chiếu:** Tất cả 804 kB. Hai pha giả định:raw+PCM+network+khác=644 kB; PCM+network+output+khác=484 kB; đỉnh 644 kB. Chưa allocator overhead.

**T29-B2 — Đối chiếu:** 50 lượt. Có đỉnh cấp phát, phân mảnh, giới hạn nội bộ và biến động; xu hướng này chỉ là mô hình để tìm lỗi.

### Tuần 30

**T30-B1 — Đối chiếu:** 90% trên bộca này. Không suy ra xác suất thành công mọi điều kiện hoặc sản phẩm đạt độ tin cậy 90%; bộca không nhất thiết đại diện ngẫu nhiên.

**T30-B2 — Đối chiếu:** Ví dụ nguồn sụt và lỗi chương trình phối hợp. Thử giảm/tách tải đúng thiết kế, so reset reason; thay chuyển động bằng event giả, giữ logic; kết luận theo bằng chứng chứ không tự khẳng định.

### Tuần 31

**T31-B1 — Đối chiếu:** 30+25+5+3+3 = 66 mm. Cần cộng thêm chiều dày vỏ ngoài nếu đang tính kích thước ngoài và kiểm tra đầu nối/dây.

**T31-B2 — Đối chiếu:** Cùng câu/nguồn phát, khoảng cách, hướng, mức âm, gain, cấu hình lấy mẫu, vị trí phòng và xử lý; đo/ghi transcript hoặc waveform với phương pháp nhất quán.

### Tuần 32

**T32-B1 — Đối chiếu:** 1 A: 0,2 V, 0,2 W. 2 A: 0,4 V, 0,8 W. Dòng gấp đôi làm nhiệt I²R gấp bốn theo R cố định.

**T32-B2 — Đối chiếu:** Không có một bảng chân chung; dùng schematic đã chốt. Bảng phải bao gồm nguồn/GND, SDA/SCL, servo signal, I²S clock/data và không bỏ rail khác điện áp.

### Tuần 33

**T33-B1 — Đối chiếu:** Đáp án cần đúng bo, sơ đồ, phiên bản, mã/lockfile, config mẫu, cách nạp/phục hồi, server, dữ liệu thử và ca nghiệm thu; không chứa khóa thật.

**T33-B2 — Đối chiếu:** So user_id, tên, preferences, thời điểm hoặc các trường cần giữ; so query có kiểm soát trên bản thử, không ghi đè dữ liệu thật để luyện.


<a id="thuat-ngu"></a>
## 9. Từ vựng cần hiểu trong ngữ cảnh

| Thuật ngữ | Nghĩa cần hiểu khi làm robot | Chỗ dễ nhầm |
|---|---|---|
| Điện áp / voltage | Chênh lệch điện thế giữa hai điểm | Không phải lượng dòng trong dây |
| Dòng điện / current | Tốc độ chuyển điện tích ròng qua tiết diện | Không bị tải tiêu hao theo nghĩa điện tích biến mất |
| Công suất / power | Tốc độ chuyển năng lượng | Khác năng lượng tổng sau một khoảng thời gian |
| GND | Mốc tham chiếu và đường hồi dòng theo thiết kế | Không luôn là nối đất bảo vệ hoặc cùng điện thế tuyệt đối mọi điểm |
| Tải / load | Phần mạch lấy năng lượng hoặc ảnh hưởng đầu ra | Dụng cụ đo cũng có thể là tải |
| Datasheet | Thông số và điều kiện do nhà sản xuất công bố | Absolute maximum không phải điểm vận hành mong muốn |
| MCU | CPU, bộ nhớ và ngoại vi trong vi điều khiển | Không chỉ là một CPU nhỏ |
| Firmware | Chương trình chạy trên thiết bị | Mã nguồn khác file firmware đã build |
| GPIO | Chân vào/ra có thể cấu hình | Chân tín hiệu không thay nguồn công suất |
| Pull-up/down | Đường điện trở tạo mức mặc định | Không biến input thành output |
| ADC | Lấy mẫu và lượng tử hóa điện áp | Độ phân giải không tự bằng độ chính xác |
| PWM | Thay tỷ lệ thời gian của chuỗi xung | Không luôn tạo một điện áp DC liên tục |
| HAL/driver | Lớp giao tiếp phần cứng/điều khiển thiết bị | Không tự sửa nguồn/chân sai |
| Framebuffer | Dữ liệu hình ảnh trong bộ nhớ | Vẽ vào buffer không luôn đồng nghĩa đã cập nhật màn hình |
| Servo vị trí | Cơ cấu có điều khiển vị trí bên trong | Lệnh góc không chứng minh góc thực |
| Task | Một đơn vị thực thi do runtime/RTOS tổ chức | Nhiều task không tự làm mọi việc nhanh hơn |
| Queue | Hàng đợi dữ liệu/sự kiện giữa các phần | Không vô hạn, cần chính sách lúc đầy |
| Timeout | Hết giới hạn chờ ở phía đang đo | Không chứng minh bên kia chưa thực hiện |
| Idempotency | Lặp cùng thao tác không tạo thêm hiệu ứng theo quy tắc đã định | Không chỉ là đặt cùng request_id rồi không xử lý nó |
| Schema | Quy tắc hình dạng/kiểu/giá trị dữ liệu | Hợp schema vẫn có thể sai ngữ nghĩa |
| Transaction | Nhóm thay đổi được quản lý nhất quán | Không giống chạy nhiều câu lệnh rời rồi hy vọng không lỗi |
| Context | Thông tin ứng dụng đưa vào một lượt gọi mô hình | Không tự bằng toàn bộ trí nhớ người dùng |
| STT | Suy văn bản từ tiếng nói | Transcript có thể sai dù âm nghe khá rõ |
| TTS | Tạo tiếng nói từ văn bản | Định dạng trả về có thể cần giải mã/chuyển đổi |
| PCM | Biểu diễn mẫu âm thanh số | Khác container WAV và khác audio nén |
| Sample rate | Số mẫu theo thời gian | Khác số byte mỗi giây và bit depth |
| Slot width | Số bit dành trên bus cho slot | Có thể lớn hơn số bit dữ liệu hữu ích |
| DMA | Chuyển dữ liệu theo cơ chế ngoại vi/bộ nhớ | Không bỏ hết trách nhiệm thời gian của phần mềm |
| Underrun | Consumer hết dữ liệu để dùng | Buffer lớn chỉ giúp trong một phạm vi |
| Overrun | Dữ liệu tới vượt khả năng lưu/xử lý | Không chữa được bằng tăng RAM vô hạn |
| Latency | Độ trễ giữa hai mốc đã định nghĩa | Không có ý nghĩa rõ nếu không nói đo từ đâu đến đâu |
| Regression | Lỗi xuất hiện lại hoặc mới do thay đổi | Cần chọn ca liên quan sau khi sửa |
| BOM | Danh sách linh kiện với mã/biến thể/số lượng | Không chỉ là danh sách tên chung |
| Reproducibility | Khả năng tái lập từ dữ liệu/quy trình đã ghi | Video demo không thay được hướng dẫn đầy đủ |

## 10. Cách tiếp tục sau bản robot đầu tiên

Chỉ chọn một hướng dựa trên nhu cầu thực: bánh xe → motor/encoder/PID; nhiều khớp → tọa độ/động học; camera → ảnh/hiệu chỉnh/độ trễ; AI offline → tài nguyên/mô hình/đánh giá; pin → năng lượng/bảo vệ/sạc/power path; PCB → schematic/layout/return path/DRC; nghe–nói đồng thời → VAD/khử tiếng vọng/streaming.

Với mỗi hướng, làm lại cùng chu trình: chọn tài liệu gốc → bài tính nhỏ → thực nghiệm riêng → xác định giới hạn → tích hợp → đo lại. Không cần học tất cả các nhánh để coi robot đầu tiên đã hoàn thành.

