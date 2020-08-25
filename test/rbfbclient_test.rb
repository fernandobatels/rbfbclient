require 'test_helper'

class RbfbclientTest < Minitest::Test
  def test_has_version
    assert !Rbfbclient::VERSION.nil?
  end

  def test_simple_conn
    conn = Rbfbclient::Connection.new({
                                        host: 'localhost',
                                        port: 3050,
                                        db_name: 'test.fdb',
                                        user: 'SYSDBA',
                                        pass: 'masterkey'
                                      })
    assert !conn.nil?
    conn.close
  end

  def test_simple_exec
    conn = Rbfbclient::Connection.new({
                                        db_name: 'test.fdb',
                                      })
    assert !conn.nil?

    begin
      conn.execute('create table fbtest (name varchar(50))')
    rescue
    end

    conn.execute('insert into fbtest (name) values (\'val test\')')

    conn.close
  end

  def test_simple_params_exec
    conn = Rbfbclient::Connection.new({
                                        db_name: 'test.fdb',
                                      })
    assert !conn.nil?

    begin
      conn.execute('create table fbtest3 (name varchar(50), city varchar(50))')
    rescue
    end

    conn.execute('insert into fbtest3 (name) values (?)', ['fulano'])
    conn.execute('insert into fbtest3 (name) values (?)', 'fulano')
    conn.execute('insert into fbtest3 (name, city) values (?, ?)', ['fulanoson', 'joinville'])
    conn.execute('insert into fbtest3 (name, city) values (?, ?)', 'fulanoson', 'joinville')

    conn.close
  end

  def test_params_support
    require 'date'

    conn = Rbfbclient::Connection.new({
                                        db_name: 'test.fdb',
                                      })
    assert !conn.nil?

    begin
      conn.execute('create table fbtest4 (a varchar(50), b int, c float, d boolean, e date, f timestamp)')
    rescue
    end

    conn.execute('insert into fbtest4 (a) values (?)', ['fulano'])
    conn.execute('insert into fbtest4 (b) values (?)', [10])
    conn.execute('insert into fbtest4 (b) values (?)', [10.to_i])
    conn.execute('insert into fbtest4 (c) values (?)', [10.20])
    conn.execute('insert into fbtest4 (a) values (?)', [nil])
    conn.execute('insert into fbtest4 (d) values (?)', [false])
    conn.execute('insert into fbtest4 (d) values (?)', [true])
    conn.execute('insert into fbtest4 (e, f) values (?, ?)', [Date.today, DateTime.now])

    conn.close
  end

  def test_query
    conn = Rbfbclient::Connection.new({
                                        db_name: 'test.fdb',
                                      })
    assert !conn.nil?

    rows = conn.query('select * from rdb$database;')
    assert_equal 1, rows.size

    rows = conn.query('select 10.20 as a, 10 as b, \'test\' as c, CURRENT_DATE as d, CURRENT_TIME as e, CURRENT_TIMESTAMP as f from rdb$database;')
    assert_equal 1, rows.size
    assert_equal 10.20, rows[0][0]
    assert_equal 10, rows[0][1]
    assert_equal 'test', rows[0][2]
    assert_equal Date.today, rows[0][3]
    now = DateTime.now
    assert_equal DateTime.new(now.year, now.month, now.day, now.hour, now.min, now.sec, '+00:00'), rows[0][4]
    assert_equal DateTime.new(now.year, now.month, now.day, now.hour, now.min, now.sec, '+00:00'), rows[0][5]

    conn.close
  end
end
