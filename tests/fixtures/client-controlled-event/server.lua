RegisterNetEvent('shop:buy')
AddEventHandler('shop:buy', function(item, price)
    print(item, price)
end)