FROM rust:1.69 as builder

# /todo でビルド
WORKDIR /todo

# ビルドに必要なイメージ
COPY Cargo.toml Cargo.toml

RUN mkdir "src"
Run echo "fn main(){}" > src/main.rs

# ビルド
RUN cargo build --release

# appにコピー
COPY ./src ./src
COPY ./templates ./templates

# 先ほどビルドした生成物のうち、アプリケーションのもののみ削除
RUN rm -f target/release/deps/todo*

# 再度ビルド
Run cargo build --release

# 新しくリリース用のイメージを用意
FROM debian:10.4

# builderイメージから todoのみをコピーして /usr/local/bin に配置
COPY --from=builder /todo/target/release/todo /usr/local/bin/todo

# コンテナ起動時にWebアプリを実行
CMD ["todo"]