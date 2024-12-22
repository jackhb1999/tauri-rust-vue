use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // up 方法用于更改数据库模式，新增表、列或索引
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        // manager
        //     .drop_table(Table::drop().table(User::Table).to_owned())
        //     .await;
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
                    .col(ColumnDef::new(User::UserCode).string().not_null().comment("用户编号"))
                    .col(ColumnDef::new(User::Username).string().not_null().comment("用户名"))
                    .col(ColumnDef::new(User::Password).string().not_null().comment("密码"))
                    .col(ColumnDef::new(User::DeptCode).string().null().comment("所属部门"))
                    .col(ColumnDef::new(User::RoleCode).string().null().comment("对应角色"))
                    .col(ColumnDef::new(User::Menus).string().null().comment("可见菜单"))
                    .to_owned(),
            )
            .await;

        manager
            .create_table(
                Table::create()
                    .table(Dept::Table).comment("部门表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Dept::DeptCode)
                            .string()
                            .not_null()
                            .primary_key().comment("部门编号"),
                    )
                    .col(ColumnDef::new(Dept::DeptName).string().not_null().comment("部门名称"))
                    .col(ColumnDef::new(Dept::ParentCode).string().null().comment("父级部门"))
                    .col(ColumnDef::new(Dept::Menus).string().null().comment("可见菜单"))
                    .to_owned(),
            )
            .await;

        manager
            .create_table(
                Table::create()
                    .table(Role::Table).comment("角色表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Role::RoleCode)
                            .string()
                            .not_null()
                            .primary_key().comment("角色编号"),
                    )
                    .col(ColumnDef::new(Role::RoleName).string().not_null().comment("角色名称"))
                    .col(ColumnDef::new(Role::Menus).string().null().comment("可见菜单"))
                    .to_owned(),
            )
            .await;

        manager
            .create_table(
                Table::create()
                    .table(Menu::Table).comment("菜单表")
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Menu::MenuCode)
                            .string()
                            .not_null()
                            .primary_key().comment("菜单编号"),
                    )
                    .col(ColumnDef::new(Menu::MenuName).string().not_null().comment("菜单名"))
                    .col(ColumnDef::new(Menu::MenuPath).string().null().comment("菜单路径"))
                    .col(ColumnDef::new(Menu::MenuIcon).string().null().comment("菜单图标"))
                    .col(ColumnDef::new(Menu::ParentCode).string().null().comment("父级菜单"))
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
    UserCode,
    Username,
    Password,
    DeptCode,
    RoleCode,
    Menus,
}

#[derive(Iden)]
enum Dept {
    Table,
    DeptCode,
    DeptName,
    ParentCode,
    Menus,
}

#[derive(Iden)]
enum Role {
    Table,
    RoleCode,
    RoleName,
    Menus,
}

#[derive(Iden)]
enum Menu {
    Table,
    MenuCode,
    MenuName,
    MenuPath,
    MenuIcon,
    ParentCode,
}