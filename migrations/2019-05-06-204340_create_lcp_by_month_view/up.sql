CREATE MATERIALIZED VIEW lcp_by_month_view as
SELECT  checks.business_id business_id,
       ((COALESCE(NULLIF(SUM(labor.pay_rate), 0), 1) /
        COALESCE(NULLIF(SUM(checks.total_price), 0), 1)
        ) * 100) as value,
        json_build_object(
        'start',
        to_timestamp(floor((extract('epoch' from checks.closed_at) / 2592000 )) * 2592000)
        AT TIME ZONE 'UTC',
        'end',
        to_timestamp(ceiling((extract('epoch' from checks.closed_at) / 2592000 )) * 2592000)
        AT TIME ZONE 'UTC' )::jsonb time_frame,
        to_timestamp(floor((extract('epoch' from checks.closed_at) / 2592000 )) * 2592000) AT TIME ZONE 'UTC' as start_time,
        to_timestamp(ceiling((extract('epoch' from checks.closed_at) / 2592000 )) * 2592000) AT TIME ZONE 'UTC' as end_time,
        concat_ws('-', checks.business_id,
                      to_timestamp(floor((extract('epoch' from checks.closed_at) / 2592000 )) * 2592000) AT TIME ZONE 'UTC',
                      to_timestamp(ceiling((extract('epoch' from checks.closed_at) / 2592000 )) * 2592000) AT TIME ZONE 'UTC'
                      ) as view_id
FROM check_view checks
INNER JOIN labor_entry labor
ON labor.business_id = checks.business_id
WHERE to_timestamp(floor((extract('epoch' from checks.closed_at) / 2592000 )) * 2592000) AT TIME ZONE 'UTC' <= labor.clock_in
AND   to_timestamp(ceiling((extract('epoch' from checks.closed_at) / 2592000 )) * 2592000) AT TIME ZONE 'UTC' >= labor.clock_out
GROUP BY start_time, end_time, checks.business_id;

CREATE UNIQUE INDEX lcb_by_month_and_business
ON lcp_by_month_view (business_id, start_time, end_time);
