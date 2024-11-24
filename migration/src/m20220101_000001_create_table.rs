use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // up 方法用于更改数据库模式，新增表、列或索引
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(User::Table).comment("用户表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key().comment("用户表主键"),
                    )
                    .col(ColumnDef::new(User::Username).string().not_null().comment("用户名"))
                    .col(ColumnDef::new(User::Password).string().not_null().comment("密码"))
                    .to_owned(),
            )
            .await


        // manager
        //     .create_table(
        //         Table::create()
        //             .table(Post::Table)
        //             .if_not_exists()
        //             .col(pk_auto(Post::Id))
        //             .col(string(Post::Title))
        //             .col(string(Post::Text))
        //             .to_owned(),
        //     )
        //     .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        todo!();

        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}

// 用户表
#[derive(Iden)]
enum User {
    Table,
    Id,
    Username,
    Password,
}