SELECT id
FROM public_user
WHERE ipaddr = ? AND name = ?
ORDER BY join_datetime DESC LIMIT 1