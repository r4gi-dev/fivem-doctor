RegisterNetEvent('player:requestData')
AddEventHandler('player:requestData', function()
    TriggerClientEvent('player:data', source)
end)