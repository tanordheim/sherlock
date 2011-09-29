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

module Sherlock #:nodoc
  module Models #:nodoc

    # Defines a snapshot containing data pushed from an agent to Sherlock.
    class Snapshot

      include MongoMapper::Document

      # Snapshots have many processes.
      many :processes, :class_name => 'Sherlock::Models::Process'

      # Snapshots have many metrics.
      many :metrics, :class_name => 'Sherlock::Models::Metric'

      # Define the fields for a snapshot.
      key :node_id, String
      key :timestamp, Time
      key :key, String

      # Validate that the snapshot has a node id set.
      validates :node_id, :presence => true
      
      # Validate that the snapshot has a timestamp set.
      validates :timestamp, :presence => true

      # Before validating or saving the model, build a unique, identifiable key
      # for this snapshot.
      before_validation :generate_key
      before_save :generate_key

      private

      # Generate a unique, identifiable key for this snapshot. This key is based
      # on the node id and the timestamp, making the key easy to calculate
      # without having to load the model first.
      def generate_key
        if self.key.blank? && !(self.node_id.blank? || self.timestamp.blank?)
          self.key = "#{node_id}-#{timestamp.strftime('%Y%m%d%H%M%S')}"
        end
      end

    end

  end
end
