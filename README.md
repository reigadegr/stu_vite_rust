## 自动生成实体类，返回前端变驼峰命名
```shell
sea-orm-cli generate entity --with-serde=both -o src/app/entities
wsl sh ./patch_sea-orm_field.sh
cargo fmt
```

## jetbrains可能会倒着执行代码
```shell
cargo fmt
wsl sh ./patch_sea-orm_field.sh
sea-orm-cli generate entity --with-serde=both -o src/app/entities
```
