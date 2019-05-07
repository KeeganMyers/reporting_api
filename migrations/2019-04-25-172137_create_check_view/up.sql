CREATE MATERIALIZED VIEW check_view as
SELECT business_id, employee_id, name, closed_at,id,
       items.total_cost total_cost, items.total_price total_price,
       concat_ws('-', business_id, id) as view_id
FROM check_tbl
JOIN (SELECT SUM(cost) total_cost, SUM(price) total_price, check_id
      FROM ordered_item
      WHERE voided = false
      GROUP BY check_id) items
ON items.check_id = id
WHERE closed = true;

CREATE UNIQUE INDEX check_by_business
ON check_view (business_id, id);
