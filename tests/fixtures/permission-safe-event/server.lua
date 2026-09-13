RegisterNetEvent('admin:giveMoney')
AddEventHandler('admin:giveMoney', function(amount)
    if not IsPlayerAceAllowed(source, 'admin') then
        return
    end

    local player = QBCore.Functions.GetPlayer(source)
    player.Functions.AddMoney('cash', amount)
end)