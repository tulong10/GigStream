# GigStream 🚀

Giải pháp thanh toán bảo lãnh (Escrow) xuyên biên giới cực nhanh và chi phí thấp dành cho Freelancer, được xây dựng trên mạng lưới Stellar (Soroban Smart Contracts).

## 💡 Ý tưởng cốt lõi (MVP)

- **Vấn đề:** Các freelancer tại các nước đang phát triển thường mất 5-10% thu nhập và phải chờ vài ngày để nhận tiền qua các kênh truyền thống.
- **Giải pháp:** GigStream sử dụng hợp đồng thông minh Soroban để tạo quỹ bảo lãnh. Tiền được khóa lại và tự động giải ngân 100% cho freelancer ngay lập tức khi công việc hoàn tất, với chi phí giao dịch chưa tới 1 cent.

## 🔗 Minh chứng triển khai (Proof of Work)

Hợp đồng đã được triển khai và tương tác thành công trên mạng Stellar Testnet:
👉 **[Xem chi tiết giao dịch trên Stellar Expert](https://stellar.expert/explorer/testnet/tx/d307e3aaf363c10c09c38190b8ab3d8eb3a42f049f75eb4be5591cfebd52b3a3)**

## 🛠 Công nghệ sử dụng

- **Smart Contract:** Soroban (Rust)
- **Blockchain:** Stellar Testnet
- **Môi trường phát triển:** Stellar IDE / Stellar CLI

## ⚙️ Các tính năng của Smart Contract

Hợp đồng quản lý luồng trạng thái an toàn giữa Khách hàng (Client) và Người làm tự do (Freelancer):

1. `init`: Khởi tạo hợp đồng, thiết lập địa chỉ ví của hai bên.
2. `fund`: Khách hàng nạp tiền vào quỹ bảo lãnh (Trạng thái chuyển sang `FUNDED`).
3. `release`: Khách hàng xác nhận nghiệm thu, hợp đồng mở khóa và giải ngân (Trạng thái chuyển sang `RELEASED`).
4. `get_status`: Truy xuất trạng thái hiện tại của hợp đồng trên chuỗi (on-chain).
