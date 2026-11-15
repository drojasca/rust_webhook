CREATE OR REPLACE FUNCTION set_status_on_zero()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.remaining = 0 THEN
        NEW.status := 'Complete';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_set_status_on_zero ON outbox;

CREATE TRIGGER trg_set_status_on_zero
BEFORE UPDATE OF remaining ON outbox
FOR EACH ROW
EXECUTE FUNCTION set_status_on_zero();