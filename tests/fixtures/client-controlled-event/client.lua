RegisterNetEvent('shop:open')
AddEventHandler('shop:open', function()
    local item = 'weapon_pistol'
    local price = 5000

    TriggerServerEvent('shop:buy', item, price)
end)