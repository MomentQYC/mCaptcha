/*
* Copyright (C) 2021  Aravinth Manivannan <realaravinth@batsense.net>
*
* This program is free software: you can redistribute it and/or modify
* it under the terms of the GNU Affero General Public License as
* published by the Free Software Foundation, either version 3 of the
* License, or (at your option) any later version.
*
* This program is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* GNU Affero General Public License for more details.
*
* You should have received a copy of the GNU Affero General Public License
* along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use actix_web::web::ServiceConfig;

pub mod auth;
pub mod mcaptcha;

pub fn services(cfg: &mut ServiceConfig) {
    use auth::*;
    use mcaptcha::*;

    cfg.service(signout);
    cfg.service(signin);
    cfg.service(signup);
    cfg.service(delete_account);

    cfg.service(add_domain);
    cfg.service(delete_domain);
    cfg.service(add_mcaptcha);
    cfg.service(delete_mcaptcha);

    cfg.service(add_levels);
    cfg.service(update_levels);
    cfg.service(delete_levels);
    cfg.service(get_levels);

    cfg.service(update_duration);
    cfg.service(get_duration);
}

#[cfg(test)]
mod tests;
