CREATE MATERIALIZED VIEW egs_by_hour_view as
SELECT checks.name as employee, checks.employee_id,checks.business_id, SUM(checks.total_price) as value,
        json_build_object(
        'start',
        to_timestamp(floor((extract('epoch' from checks.closed_at) / 3600 )) * 3600)
        AT TIME ZONE 'UTC',
        'end',
        to_timestamp(ceiling((extract('epoch' from checks.closed_at) / 3600 )) * 3600)
        AT TIME ZONE 'UTC')::jsonb time_frame,
        to_timestamp(floor((extract('epoch' from checks.closed_at) / 3600 )) * 3600) AT TIME ZONE 'UTC' start_time,
        to_timestamp(ceiling((extract('epoch' from checks.closed_at) / 3600 )) * 3600) AT TIME ZONE 'UTC' end_time,
        concat_ws('-', checks.business_id, checks.employee_id, 
                       to_timestamp(floor((extract('epoch' from checks.closed_at) / 3600 )) * 3600) AT TIME ZONE 'UTC',
                       to_timestamp(ceiling((extract('epoch' from checks.closed_at) / 3600 )) * 3600) AT TIME ZONE 'UTC'
                    ) as view_id
FROM check_view checks
GROUP BY start_time, end_time, checks.business_id, checks.name, checks.employee_id;

CREATE UNIQUE INDEX egs_by_hour_and_business
ON egs_by_hour_view (business_id, employee_id, start_time, end_time);
