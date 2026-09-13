RegisterNetEvent('bank:withdraw')
AddEventHandler('bank:withdraw', function(amount)
    local player = QBCore.Functions.GetPlayer(source)

    player.Functions.RemoveMoney('bank', amount)
end)