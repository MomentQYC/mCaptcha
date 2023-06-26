-- SPDX-FileCopyrightText: 2023 Aravinth Manivannan <realaravinth@batsense.net>
--
-- SPDX-License-Identifier: AGPL-3.0-or-later

--  mark a notification as read
UPDATE mcaptcha_notifications
    SET read = TRUE
WHERE 
    mcaptcha_notifications.id = $1
AND
    mcaptcha_notifications.rx = (
        SELECT
            id
        FROM
            mcaptcha_users
        WHERE
        name = $2
    );
