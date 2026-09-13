RegisterNetEvent('player:requestData')
AddEventHandler('player:requestData', function(request)
    local data = {
        name = 'test'
    }

    TriggerClientEvent('player:data', source, data)
end)