# My editor can run "rake" with a single button press. This is a shim to make that run make instead.
task :default do
  system("make") || exit(1)
  puts
  puts "You might want to do `alias rake=make` to avoid using this wrapper."
end
