# A new Firebird Extension Library for Ruby

A new version of [Firebird Extension Library for Ruby](https://github.com/rowland/fb) using [Rust instead of C](https://github.com/fernandobatels/rsfbclient) for binding of firebird API. 

## Goals

The main goal is provide the same API of [Firebird Extension Library](https://github.com/rowland/fb). Maybe some methods or approaches will change. 

- [x] Connections
- [ ] Transactions
- [x] Execute statements
- [x] Query statements
- [ ] Cursor
- [ ] Replace the [Firebird Extension Library](https://github.com/rowland/fb) on [ActiveRecord Firebird Adapter](https://github.com/FabioMR/firebird_adapter)

## Installation

Add this line to your application's Gemfile:

```ruby
gem 'rbfbclient'
```

And then execute:

    $ bundle

Or install it yourself as:

    $ gem install rbfbclient

## Development

After checking out the repo, run `bin/setup` to install dependencies. Then, run `rake spec` to run the tests. You can also run `bin/console` for an interactive prompt that will allow you to experiment.

To install this gem onto your local machine, run `bundle exec rake install`. To release a new version, update the version number in `version.rb`, and then run `bundle exec rake release`, which will create a git tag for the version, push git commits and tags, and push the `.gem` file to [rubygems.org](https://rubygems.org).

## Contributing

Bug reports and pull requests are welcome on GitHub at https://github.com/fernandobatels/rbfbclient.

## License

The gem is available as open source under the terms of the [MIT License](https://opensource.org/licenses/MIT).
