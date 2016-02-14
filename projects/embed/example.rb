threads = []

10.times do
  threads << Thread.new do
    count = 0

    1_500_000.times do
      count += 1
    end

    count
  end
end

threads.each do |t| 
  puts "Thread finished with count=#{t.value}"
end

puts "done!"
