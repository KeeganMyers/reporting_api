CREATE MATERIALIZED VIEW fcp_by_day_view as
SELECT SUM(checks.total_price) total_price, business_id,
       ((COALESCE(NULLIF(SUM(checks.total_cost), 0), 1) /
        COALESCE(NULLIF(SUM(checks.total_price), 0), 1)
        ) * 100) as value,
        json_build_object(
        'start',
        to_timestamp(floor((extract('epoch' from checks.closed_at) / 86400 )) * 86400)
        AT TIME ZONE 'UTC',
        'end',
        to_timestamp(ceiling((extract('epoch' from checks.closed_at) / 86400 )) * 86400)
        AT TIME ZONE 'UTC')::jsonb time_frame,
        to_timestamp(floor((extract('epoch' from checks.closed_at) / 86400 )) * 86400) AT TIME ZONE 'UTC' as start_time, 
        to_timestamp(ceiling((extract('epoch' from checks.closed_at) / 86400 )) * 86400) AT TIME ZONE 'UTC' as end_time,
        concat_ws('-', business_id, 
                       to_timestamp(floor((extract('epoch' from checks.closed_at) / 86400 )) * 86400) AT TIME ZONE 'UTC', 
                       to_timestamp(ceiling((extract('epoch' from checks.closed_at) / 86400 )) * 86400) AT TIME ZONE 'UTC'
                    ) as  view_id
FROM check_view checks
GROUP BY start_time, end_time, business_id;

CREATE UNIQUE INDEX fcp_by_day_and_business
ON fcp_by_day_view (business_id, start_time, end_time);
