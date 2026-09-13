RegisterNetEvent('event:registered')

AddEventHandler('event:different', function(amount)
    local player = QBCore.Functions.GetPlayer(source)
    player.Functions.AddMoney('cash', amount)
end)