RegisterNetEvent('bank:withdraw')
AddEventHandler('bank:withdraw', function(amount)
    local player = QBCore.Functions.GetPlayer(source)
    player.Functions.RemoveMoney('bank', amount)
end)

RegisterNetEvent('admin:giveMoney')
AddEventHandler('admin:giveMoney', function(amount)
    if not QBCore.Functions.HasPermission(source, 'admin') then
        return
    end

    local player = QBCore.Functions.GetPlayer(source)
    player.Functions.AddMoney('cash', amount)
end)