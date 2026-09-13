RegisterNetEvent('admin:giveMoney')
AddEventHandler('admin:giveMoney', function(amount)
    local player = QBCore.Functions.GetPlayer(source)

    player.Functions.AddMoney('cash', amount)
end)