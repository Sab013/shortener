use crate::api::{
    dto::{CreateLinkRequest, CreateLinkResponse},
    handlers,
};
use crate::domain::LinkStats;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::create_short_link,
        handlers::redirect,
        handlers::get_stats
    ),
    components(
        schemas(
            CreateLinkRequest,
            CreateLinkResponse,
            LinkStats,
        )
    ),
    info(
        title = "QuickLink API",
        version = "1.0.0",
        description = "API для сокращения URL-адресов\n\
        ### Возможности\n\
        - Создание коротких ссылок\n\
        - Перенаправление по коротким ссылкам\n\
        - Просмотр статистики использования\n\
        \n\
        ### Использование\n\
        1. Создайте короткую ссылку через POST /slug\n\
        2. Используйте полученный slug для редиректа\n\
        3. Отслеживайте статистику через GET /{slug}/stats",
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        ),
        contact(
            name = "Vitaly Vasiltsov",
            email = "dev9900195@gmail.com",
            url = "https://github.com/Sab013/shortener",
        )
    ),
    tags(
        (name = "shortener", description = "URL Shortener endpoints. Позволяет создавать и управлять короткими ссылками.")
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, _openapi: &mut utoipa::openapi::OpenApi) {
        // Здесь можно добавить дополнительные модификации OpenAPI схемы
    }
}
