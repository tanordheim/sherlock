# encoding: utf-8

# Copyright 2011 Binary Marbles.
# 
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
# 
# http://www.apache.org/licenses/LICENSE-2.0
# 
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

require 'rubygems'
require 'bundler/setup'
require 'sinatra/base'
require 'haml'

$:.unshift File.expand_path(File.join(File.dirname(__FILE__), 'lib'))
require 'sherlock'
require 'sherlock/sinatra'

# Require all Sinatra controllers.
Dir[File.expand_path(File.join(File.dirname(__FILE__), 'app', 'controllers', '*.rb'))].each do |f|
  require f
end
