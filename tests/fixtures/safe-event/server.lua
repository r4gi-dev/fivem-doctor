RegisterNetEvent('admin:giveMoney')
AddEventHandler('admin:giveMoney', function(amount)
    if type(amount) ~= 'number' or amount <= 0 then
        return
    end

    if not QBCore.Functions.HasPermission(source, 'admin') then
        return
    end

    local player = QBCore.Functions.GetPlayer(source)
    player.Functions.AddMoney('cash', amount)
end)