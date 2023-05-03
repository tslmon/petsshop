

--
--  users
--

CREATE TABLE IF NOT EXISTS users (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    fname character varying(255) NOT NULL,
    lname character varying(255) NOT NULL,
    gender character varying(255) NOT NULL,
    email character varying(255) NOT NULL,
    phone_number character varying(255) NOT NULL,
    address text,
    type character varying(255) NOT NULL DEFAULT 'USER',
    user_name character varying(255) NOT NULL,
    pwd character varying(255) NOT NULL,
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_users PRIMARY KEY (id)
);

SELECT diesel_manage_updated_at('users');

--
-- user_identifies
--

CREATE TABLE IF NOT EXISTS identifies (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    user_id character varying(255) NOT NULL,
    usr_name character varying(255) NOT NULL,
    usr_pwd character varying(255) NOT NULL,
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_identifies PRIMARY KEY (id),
    CONSTRAINT fkey_identifies_users FOREIGN KEY (user_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE          
);

SELECT diesel_manage_updated_at('identifies');


--
-- orders
--

CREATE TABLE IF NOT EXISTS usr_orders (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    user_id character varying(255) NOT NULL,
    order_date timestamp without time zone NOT NULL DEFAULT now(),
    status character varying(255) NOT NULL,
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_usr_orders PRIMARY KEY (id),
    CONSTRAINT fkey_usr_orders_users FOREIGN KEY (user_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('usr_orders');

--
-- categories
--

CREATE TABLE IF NOT EXISTS categories (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    name character varying(255) NOT NULL,
    parent character varying(255) NOT NULL,
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_categories PRIMARY KEY (id)
);

SELECT diesel_manage_updated_at('categories');

--
-- products
--

CREATE TABLE IF NOT EXISTS products (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    name character varying(255) NOT NULL,
    description text,
    image text NOT NULL,
    price bigint NOT NULL,
    quantity bigint NOT NULL,
    category_id character varying(255) NOT NULL, 
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_products PRIMARY KEY (id),
    CONSTRAINT fkey_products_categories FOREIGN KEY (category_id)
        REFERENCES categories (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

SELECT diesel_manage_updated_at('products');

--
-- order_items
--

CREATE TABLE IF NOT EXISTS order_items (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),    
    order_id character varying(255) NOT NULL,
    product_id character varying(255) NOT NULL,
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_order_items PRIMARY KEY (id),    
    CONSTRAINT fkey_order_items_usr_orders FOREIGN KEY (order_id)
        REFERENCES usr_orders (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,  
    CONSTRAINT fkey_order_items_products FOREIGN KEY (product_id)
        REFERENCES products (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE             
);

SELECT diesel_manage_updated_at('order_items');

--
-- payments
--

CREATE TABLE IF NOT EXISTS payments (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    order_id character varying(255) NOT NULL,
    amount character varying(255) NOT NULL,
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_payments PRIMARY KEY (id),
    CONSTRAINT fkey_payments_usr_orders FOREIGN KEY (order_id)
        REFERENCES usr_orders (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE    
);

SELECT diesel_manage_updated_at('payments');

--
-- comments
--

CREATE TABLE IF NOT EXISTS comments (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    user_id character varying(255) NOT NULL,
    product_id character varying(255) NOT NULL,
    comment text,
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_comments PRIMARY KEY (id),
    CONSTRAINT fkey_comments_users FOREIGN KEY (user_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,   
    CONSTRAINT fkey_comments_products FOREIGN KEY (product_id)
        REFERENCES products (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE               
);

SELECT diesel_manage_updated_at('comments');

--
-- cards
--

CREATE TABLE IF NOT EXISTS cards (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    user_id character varying(255) NOT NULL,
    product_id character varying(255) NOT NULL,
    created_by character varying(255) NOT NULL,      
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,      
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_cards PRIMARY KEY (id),
    CONSTRAINT fkey_cards_users FOREIGN KEY (user_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE,   
    CONSTRAINT fkey_cards_products FOREIGN KEY (product_id)
        REFERENCES products (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE               
);

SELECT diesel_manage_updated_at('cards');

--
-- ////// Aggregations. //////
--

--
-- users Aggregations
--

CREATE TABLE IF NOT EXISTS user_aggregations (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    user_id character varying(255) NOT NULL,
    orders bigint NOT NULL DEFAULT 0,  
    created_by character varying(255) NOT NULL,    
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,        
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_user_aggregations PRIMARY KEY (id),
    CONSTRAINT fkey_user_aggregations_users FOREIGN KEY (user_id)
        REFERENCES users (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);


CREATE INDEX idx_user_aggregations_orders
    ON user_aggregations USING btree
    (orders DESC NULLS FIRST);

SELECT diesel_manage_updated_at('user_aggregations');

--
-- categories Aggregations
--

CREATE TABLE IF NOT EXISTS category_aggregations (
    id character varying(255) NOT NULL DEFAULT uuid_generate_v4(),
    category_id character varying(255) NOT NULL,
    products bigint NOT NULL DEFAULT 0,
    created_by character varying(255) NOT NULL,  
    created_at timestamp without time zone NOT NULL DEFAULT now(),
    updated_by character varying(255) NOT NULL,     
    updated_at timestamp without time zone NOT NULL DEFAULT now(),
    CONSTRAINT pkey_category_aggregations PRIMARY KEY (id),
    CONSTRAINT fkey_category_aggregations_categories FOREIGN KEY (category_id)
        REFERENCES categories (id) MATCH SIMPLE
        ON UPDATE CASCADE
        ON DELETE CASCADE
);

CREATE INDEX idx_category_aggregations_products
    ON category_aggregations USING btree
    (products DESC NULLS FIRST);

--
--  ////////////////// aggragation main
--

--
--  user Aggregations main function.
--

CREATE OR REPLACE FUNCTION user_aggregations_main()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    insert into user_aggregations (user_id, created_by, updated_by) values (NEW.id, NEW.created_by, NEW.updated_by);
  ELSIF (TG_OP = 'DELETE') THEN
    delete from user_aggregations where user_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

--
--  user trigger for user_aggregations.
--

CREATE OR REPLACE TRIGGER user_aggregations_main 
    AFTER INSERT OR DELETE
    ON users
    FOR EACH ROW
    EXECUTE PROCEDURE user_aggregations_main();


--
--  category Aggregations main function.
--

CREATE OR REPLACE FUNCTION category_aggregations_main()
    RETURNS trigger
    LANGUAGE 'plpgsql'
    COST 100
    VOLATILE NOT LEAKPROOF
AS $BODY$
begin
  IF (TG_OP = 'INSERT') THEN
    insert into category_aggregations (category_id, created_by, updated_by) values (NEW.id, NEW.created_by, NEW.updated_by);
  ELSIF (TG_OP = 'DELETE') THEN
    delete from category_aggregations where category_id = OLD.id;
  END IF;
  return null;
end
$BODY$;

--
--  category trigger for source_aggregations.
--

CREATE OR REPLACE TRIGGER category_aggregations_main 
    AFTER INSERT OR DELETE
    ON categories
    FOR EACH ROW
    EXECUTE PROCEDURE category_aggregations_main();    
