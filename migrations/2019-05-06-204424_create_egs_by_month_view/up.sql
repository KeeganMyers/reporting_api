CREATE MATERIALIZED VIEW egs_by_month_view as
SELECT checks.name as employee, checks.employee_id,checks.business_id, SUM(checks.total_price) as value,
        json_build_object(
        'start',
        to_timestamp(floor((extract('epoch' from checks.closed_at) / 2592000 )) * 2592000)
        AT TIME ZONE 'UTC',
        'end',
        to_timestamp(ceiling((extract('epoch' from checks.closed_at) / 2592000 )) * 2592000)
        AT TIME ZONE 'UTC')::jsonb time_frame,
        to_timestamp(floor((extract('epoch' from checks.closed_at) / 2592000 )) * 2592000) AT TIME ZONE 'UTC' as start_time,
        to_timestamp(ceiling((extract('epoch' from checks.closed_at) / 2592000 )) * 2592000) AT TIME ZONE 'UTC' as end_time,
        concat_ws('-', checks.business_id, checks.employee_id, 
                       to_timestamp(floor((extract('epoch' from checks.closed_at) / 2592000 )) * 2592000) AT TIME ZONE 'UTC',
                       to_timestamp(ceiling((extract('epoch' from checks.closed_at) / 2592000 )) * 2592000) AT TIME ZONE 'UTC'
                    ) as view_id
FROM check_view checks
GROUP BY start_time, end_time, checks.business_id, checks.name, checks.employee_id;

CREATE UNIQUE INDEX egs_by_month_and_business
ON egs_by_month_view (business_id, employee_id, start_time, end_time);
