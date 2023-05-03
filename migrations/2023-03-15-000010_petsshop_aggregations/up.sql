
--
--  orders
--
CREATE FUNCTION user_aggregations_usr_orders()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update user_aggregations 
    set usr_orders = usr_orders + 1
    where user_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update user_aggregations sa
    set usr_orders = usr_orders - 1
    where user_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER user_aggregations_usr_orders
    AFTER INSERT OR DELETE
    ON usr_orders
    FOR EACH ROW
    EXECUTE PROCEDURE user_aggregations_usr_orders();


--
--  products
--
CREATE FUNCTION category_aggregations_products()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    update category_aggregations 
    set products = products + 1
    where category_id = NEW.id;
  ELSIF (TG_OP = 'DELETE') THEN
    update category_aggregations sa
    set products = products - 1
    where category_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

CREATE TRIGGER category_aggregations_products
    AFTER INSERT OR DELETE
    ON products
    FOR EACH ROW
    EXECUTE PROCEDURE category_aggregations_products();